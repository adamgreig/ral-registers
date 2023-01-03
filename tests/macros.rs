//! Tests that read, write, modify, and reset macros
//! work with
//!
//! - scalar register syntax
//! - array register syntax
//!
//! when supplied with
//!
//! - a reference to a register block.
//! - an "instance" type that derefs to a register block.
//! - a pointer to a register block.

#![allow(non_upper_case_globals, non_snake_case)] // Macro conventions.

use ral_registers as ral;

/// A peripheral module.
mod periph {
    #[repr(C)]
    pub struct RegisterBlock {
        pub MY_SCALAR: ral_registers::RWRegister<u32>,
        pub MY_ARRAY: [ral_registers::RWRegister<u32>; 3],
    }

    pub mod MY_SCALAR {
        pub mod FIELD_A {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x7F << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        pub mod FIELD_B {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }

    /// The register array module resembles the register module for
    /// a scalar register. The macros distinguish the index
    /// operator from the register name to look up items in
    /// this module.
    pub mod MY_ARRAY {
        // For testing convenience, we're pretending that MY_ARRAY
        // has the same fields as MY_SCALAR.
        pub use super::MY_SCALAR::*;
    }

    /// Reset values are always expressed as a scalar, no matter
    /// the register plurality.
    pub struct ResetValues {
        pub MY_SCALAR: u32,
        pub MY_ARRAY: u32,
    }

    pub mod INST {
        pub const reset: super::ResetValues = super::ResetValues {
            MY_SCALAR: 42,
            MY_ARRAY: 42,
        };
    }
}

fn register_block() -> periph::RegisterBlock {
    // Safety: bitpattern of zero is fine.
    use std::mem::MaybeUninit;
    unsafe { MaybeUninit::zeroed().assume_init() }
}

struct Instance<'a> {
    rb: &'a periph::RegisterBlock,
}

impl<'a> Instance<'a> {
    fn new(rb: &'a periph::RegisterBlock) -> Self {
        Self { rb }
    }
}

impl std::ops::Deref for Instance<'_> {
    type Target = periph::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        self.rb
    }
}

/// Defines the base cases for read_reg.
///
/// The goal is to have one macro path that supports both scalar
/// and array tests cases. To achieve that, there's always a register
/// identifier. There's optionally a bracket that has an offset.
macro_rules! read_reg_test_cases {
    ($instance:expr, $register:ident $([$offset:expr])*) => {
        // Setup:
        (*$instance).$register $([$offset])*.write(u32::MAX);

        assert_eq!(ral::read_reg!(periph, $instance, $register $([$offset])*), u32::MAX, "Direct read");

        assert_eq!(ral::read_reg!(periph, $instance, $register $([$offset])*, FIELD_A), 0x7F, "Individual field read (A)");
        assert_eq!(ral::read_reg!(periph, $instance, $register $([$offset])*, FIELD_B), 0b11, "Individual field read (B)");

        assert_eq!(
            ral::read_reg!(periph, $instance, $register $([$offset])*, FIELD_A, FIELD_B),
            (0x7F, 0b11),
            "Tuple field reads"
        );

        assert!(ral::read_reg!(
            periph,
            $instance,
            $register $([$offset])*,
            FIELD_A == 0x7F
        ), "Boolean expressions");
    };
}

#[test]
fn read_scalar_ref() {
    let rb = register_block();
    read_reg_test_cases!(&rb, MY_SCALAR);
}

#[test]
fn read_scalar_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    read_reg_test_cases!(inst, MY_SCALAR);
}

#[test]
fn read_scalar_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        read_reg_test_cases!(ptr, MY_SCALAR);
    }
}

#[test]
fn read_array_ref() {
    let rb = register_block();
    read_reg_test_cases!(&rb, MY_ARRAY[1]);
}

#[test]
fn read_array_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    read_reg_test_cases!(inst, MY_ARRAY[1]);
}

#[test]
fn read_array_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        read_reg_test_cases!(ptr, MY_ARRAY[1]);
    }
}

#[should_panic]
#[test]
fn read_array_out_of_bounds() {
    let rb = register_block();
    ral::read_reg!(periph, &rb, MY_ARRAY[42]);
}

/// Base test cases for write_reg.
///
/// See [read_reg_test_cases] for more information.
macro_rules! write_reg_test_cases {
    ($instance:expr, $register:ident $([$offset:expr])*) => {
        ral::write_reg!(periph, $instance, $register $([$offset])*, FIELD_A: u32::MAX);
        assert_eq!((*$instance).$register $([$offset])*.read(), 0x7F, "1:1 write:field (A)");
        ral::write_reg!(periph, $instance, $register $([$offset])*, FIELD_B: u32::MAX);
        assert_eq!((*$instance).$register $([$offset])*.read(), 0b11 << 27, "1:1 write:field (B)");

        ral::write_reg!(
            periph,
            $instance,
            $register $([$offset])*,
            FIELD_A: u32::MAX,
            FIELD_B: u32::MAX
        );
        assert_eq!((*$instance).$register $([$offset])*.read(), (0b11 << 27) | 0x7F, "1:N write:field");

        ral::write_reg!(periph, $instance, $register $([$offset])*, 0xAAAAAAAA);
        assert_eq!((*$instance).$register $([$offset])*.read(), 0xAAAAAAAA, "Direct write");

        // Make sure that local variables mask, offset don't shadow macro details.
        #[deny(warnings)]
        {
            let mask = 7;
            let offset = 8;
            ral::write_reg!(periph, $instance, $register $([$offset])*, FIELD_A: mask, FIELD_B: offset);
        }
    };
}

#[test]
fn write_scalar_ref() {
    let rb = register_block();
    write_reg_test_cases!(&rb, MY_SCALAR);
}

#[test]
fn write_scalar_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    write_reg_test_cases!(inst, MY_SCALAR);
}

#[test]
fn write_scalar_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        write_reg_test_cases!(ptr, MY_SCALAR);
    }
}

#[test]
fn write_array_ref() {
    let rb = register_block();
    write_reg_test_cases!(&rb, MY_ARRAY[1]);
}

#[test]
fn write_array_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    write_reg_test_cases!(inst, MY_ARRAY[1]);
}

#[test]
fn write_array_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        write_reg_test_cases!(ptr, MY_ARRAY[1]);
    }
}

#[should_panic]
#[test]
fn write_array_out_of_bounds() {
    let rb = register_block();
    ral::write_reg!(periph, &rb, MY_ARRAY[42], 7);
}

/// Base test cases for modify_reg.
///
/// See [read_reg_test_cases] for more information.
macro_rules! modify_reg_test_cases {
    ($instance:expr, $register:ident $([$offset:expr])*) => {
        ral::modify_reg!(periph, $instance, $register $([$offset])*, FIELD_A: u32::MAX);
        assert_eq!((*$instance).$register $([$offset])*.read(), 0x7F, "RMW individual fields (A)");
        ral::modify_reg!(periph, $instance, $register $([$offset])*, FIELD_B: u32::MAX);
        assert_eq!((*$instance).$register $([$offset])*.read(), 0x7F | (0b11 << 27), "RMW individual fields (B)");

        ral::modify_reg!(periph, $instance, $register $([$offset])*, FIELD_A: 2, FIELD_B: 2);
        assert_eq!((*$instance).$register $([$offset])*.read(), 2 | (2 << 27), "RMW multiple fields");

        ral::modify_reg!(periph, $instance, $register $([$offset])*, |reg| {
            assert_eq!(reg, 2 | (2 << 27));
            reg | u32::MAX
        });
        assert_eq!((*$instance).$register $([$offset])*.read(), u32::MAX, "RMW whole register");

        // Make sure that local variables mask, offset don't shadow macro details.
        #[deny(warnings)]
        {
            let mask = 7;
            let offset = 8;
            ral::modify_reg!(periph, $instance, $register $([$offset])*, FIELD_A: mask, FIELD_B: offset);
        }
    };
}

#[test]
fn modify_scalar_ref() {
    let rb = register_block();
    modify_reg_test_cases!(&rb, MY_SCALAR);
}

#[test]
fn modify_scalar_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    modify_reg_test_cases!(inst, MY_SCALAR);
}

#[test]
fn modify_scalar_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        modify_reg_test_cases!(ptr, MY_SCALAR);
    }
}

#[test]
fn modify_array_ref() {
    let rb = register_block();
    modify_reg_test_cases!(&rb, MY_ARRAY[1]);
}

#[test]
fn modify_array_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    modify_reg_test_cases!(inst, MY_ARRAY[1]);
}

#[test]
fn modify_array_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        modify_reg_test_cases!(ptr, MY_ARRAY[1]);
    }
}

#[should_panic]
#[test]
fn modify_array_out_of_bounds() {
    let rb = register_block();
    ral::modify_reg!(periph, &rb, MY_ARRAY[42], |_| 7);
}

/// Base test cases for reset_reg.
///
/// See [read_reg_test_cases] for more information.
macro_rules! reset_reg_test_cases {
    ($instance:expr, $register:ident $([$offset:expr])*) => {
        (*$instance).$register $([$offset])*.write(u32::MAX);
        ral::reset_reg!(periph, $instance, INST, $register $([$offset])*);
        assert_eq!((*$instance).$register $([$offset])*.read(), 42, "Entire register");

        (*$instance).$register $([$offset])*.write(u32::MAX);
        ral::reset_reg!(periph, $instance, INST, $register $([$offset])*, FIELD_B);
        assert_eq!((*$instance).$register $([$offset])*.read(), u32::MAX & !(0b11 << 27), "Field in register (B)");
        ral::reset_reg!(periph, $instance, INST, $register $([$offset])*, FIELD_A);
        assert_eq!(
            (*$instance).$register $([$offset])*.read(),
            u32::MAX & !(0b11 << 27) & !0x7F | 42,
            "Field in register (A)"
        );

        (*$instance).$register $([$offset])*.write(u32::MAX);
        ral::reset_reg!(periph, $instance, INST, $register $([$offset])*, FIELD_B, FIELD_A);
        assert_eq!(
            (*$instance).$register $([$offset])*.read(),
            u32::MAX & !(0b11 << 27) & !0x7F | 42,
            "Fields in register"
        );
    };
}

#[test]
fn reset_scalar_ref() {
    let rb = register_block();
    reset_reg_test_cases!(&rb, MY_SCALAR);
}

#[test]
fn reset_scalar_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    reset_reg_test_cases!(inst, MY_SCALAR);
}

#[test]
fn reset_scalar_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        reset_reg_test_cases!(ptr, MY_SCALAR);
    }
}

#[test]
fn reset_array_ref() {
    let rb = register_block();
    reset_reg_test_cases!(&rb, MY_ARRAY[1]);
}

#[test]
fn reset_array_deref() {
    let rb = register_block();
    let inst = Instance::new(&rb);
    reset_reg_test_cases!(inst, MY_ARRAY[1]);
}

#[test]
fn reset_array_ptr() {
    let ptr: *const _ = &register_block();
    unsafe {
        reset_reg_test_cases!(ptr, MY_ARRAY[1]);
    }
}

#[should_panic]
#[test]
fn reset_array_out_of_bounds() {
    let rb = register_block();
    ral::reset_reg!(periph, &rb, INST, MY_ARRAY[42]);
}
