//! Testing corner cases.

#![allow(non_upper_case_globals, non_snake_case)] // Macro conventions.

use ral_registers as ral;

mod periph {
    #[repr(C)]
    pub struct RegisterBlock {
        /// Multi-dimensional arrays.
        #[allow(clippy::type_complexity)] // Intentionally complex type.
        pub DEEP_LEARNING: [[[[[[[[ral_registers::RWRegister<u32>; 1]; 2]; 3]; 4]; 5]; 6]; 7]; 8],
    }

    pub mod DEEP_LEARNING {
        pub mod GRADIENT {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x1F << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }

    pub struct ResetValues {
        pub DEEP_LEARNING: u32,
    }

    pub mod INST {
        pub const reset: super::ResetValues = super::ResetValues { DEEP_LEARNING: 42 };
    }
}

fn register_block() -> periph::RegisterBlock {
    // Safety: bitpattern of zero is fine.
    use std::mem::MaybeUninit;
    unsafe { MaybeUninit::zeroed().assume_init() }
}

#[test]
fn read_deep_array() {
    let rb = register_block();
    rb.DEEP_LEARNING[7][6][5][4][3][2][1][0].write(u32::MAX);
    let gradient = ral::read_reg!(periph, &rb, DEEP_LEARNING[7][6][5][4][3][2][1][0], GRADIENT);
    assert_eq!(gradient, 0x1F);
}

#[test]
fn write_deep_array() {
    let rb = register_block();
    ral::write_reg!(periph, &rb, DEEP_LEARNING[7][6][5][4][3][2][1][0], 23);
    assert_eq!(rb.DEEP_LEARNING[7][6][5][4][3][2][1][0].read(), 23);
}

#[test]
fn modify_deep_array() {
    let rb = register_block();
    ral::modify_reg!(periph, &rb, DEEP_LEARNING[7][6][5][4][3][2][1][0], GRADIENT: 42);
    assert_eq!(
        rb.DEEP_LEARNING[7][6][5][4][3][2][1][0].read(),
        (42 & 0x1F) << 3
    );
}

#[test]
fn reset_deep_array() {
    let rb = register_block();
    ral::reset_reg!(periph, &rb, INST, DEEP_LEARNING[7][6][5][4][3][2][1][0]);
    assert_eq!(rb.DEEP_LEARNING[7][6][5][4][3][2][1][0].read(), 42);
}
