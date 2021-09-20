//! Test the read, write, modify macros.

use ral_registers as ral;

struct DummyRegisterBlock {
    register: ral::RWRegister<u32>,
}

mod register {
    #![allow(non_upper_case_globals, non_snake_case)] // Macro conventions.

    pub mod field_foo {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x7 << offset;
        pub mod RW {}
        pub mod W {}
        pub mod R {}
    }
}

fn zeroed_register_block() -> DummyRegisterBlock {
    use core::mem::MaybeUninit;
    let register_block = MaybeUninit::zeroed();
    // Safety: 0 is a safe bitpattern
    unsafe { register_block.assume_init() }
}

#[test]
fn register_read() {
    let register_block = zeroed_register_block();
    register_block.register.write(0b111 << 10);
    assert_eq!(
        0x7,
        ral::read_reg!(self, &register_block, register, field_foo)
    );
}

#[test]
fn register_write() {
    let register_block = zeroed_register_block();
    ral::write_reg!(self, &register_block, register, field_foo: 5);
    assert_eq!(5 << 10, register_block.register.read());
}

#[test]
fn register_modify() {
    let register_block = zeroed_register_block();
    register_block.register.write(1 << 10);
    ral::modify_reg!(self, &register_block, register, field_foo: 6);
    assert_eq!(6 << 10, register_block.register.read());
}

/// Demonstrates that a local variable, 'mask'
/// doesn't affect the macro's behavior.
///
/// This is the same test as register_modify(), but
/// with the number '6' in a variable called 'mask'.
#[deny(warnings)] // Promotes "unused variable: mask" into an error. Should no longer happen.
#[test]
fn register_unused_mask_modify() {
    let register_block = zeroed_register_block();
    register_block.register.write(1 << 10);
    let mask = 6;
    ral::modify_reg!(self, &register_block, register, field_foo: mask);
    assert_eq!(6 << 10, register_block.register.read());
}

/// Same test as above, but with a variable called
/// 'offset' instead of 'mask'.
#[deny(warnings)]
#[test]
fn register_unused_offset_modify() {
    let register_block = zeroed_register_block();
    register_block.register.write(1 << 10);
    let offset = 6;
    ral::modify_reg!(self, &register_block, register, field_foo: offset);
    assert_eq!(6 << 10, register_block.register.read());
}

/// Same as above test, but using the 'write' macro
/// instead of 'modify'.
#[deny(warnings)]
#[test]
fn register_unused_mask_write() {
    let register_block = zeroed_register_block();
    register_block.register.write(1 << 10);
    let mask = 6;
    ral::write_reg!(self, &register_block, register, field_foo: mask);
    assert_eq!(6 << 10, register_block.register.read());
}

/// Same test as above, but with a variable called
/// 'offset' instead of 'mask'.
#[deny(warnings)]
#[test]
fn register_unused_offset_write() {
    let register_block = zeroed_register_block();
    register_block.register.write(1 << 10);
    let offset = 6;
    ral::write_reg!(self, &register_block, register, field_foo: offset);
    assert_eq!(6 << 10, register_block.register.read());
}
