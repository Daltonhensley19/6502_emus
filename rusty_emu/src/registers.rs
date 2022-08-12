#![allow(unused)]

use bitflags::bitflags;

bitflags! {
    struct StatusReg: u8 {
        /// Carry/Borrow flag: used in math and rotate operations.
        const C = 0b0000_0001;
        /// Zero flag: Set if the register's value is equal to the input value.
        const Z = 0b0000_0010;
        /// Interrupt flag: Disables interrupts while set.
        const I = 0b0000_0100;
        /// Decimal mode flag: mathematical instructions will treat the inputs and outputs as decimal numbers.
        const D = 0b0000_1000;
        /// Break flag: Set if an interrupt request has been triggered by a BRK instruction.
        const B = 0b0001_0000;
        /// Always set.
        const UNUSED = 0b0010_0000;
        /// Overflow Arithmetic flag: Set if a signed overflow occurred during addition or subtraction.
        const V = 0b0100_0000;
        /// Negative flag: Set if the register's value is less than the input value
        const N = 0b1000_0000;
    }
}

/// `Registers` is an abstraction for the emulated local storage in the form of cpu registers.
#[derive(Debug)]
pub struct Registers {
    pc: u16,
    acc: u8,
    x: u8,
    y: u8,
    sp: u8,
    sr: StatusReg,
}

impl Registers {
    /// Public method used to initalize the cpu registers.
    pub fn initalize() -> Self {
        let sr = StatusReg::all();

        Self {
            pc: 0,
            acc: 0,
            x: 0,
            y: 0,
            sp: 0,
            sr,
        }
    }
}
