#![allow(unused)]

use crate::memory::EmuMemory;
use crate::registers::Registers;
use crate::utility::RomLoader;

#[derive(Clone, Copy, Debug)]
enum AddressingMode {
    Accumulator,
    Implied,
    Immediate(u8),
    Absolute(u16),
    ZPage(u8),
    ZPageX((u8, u8)),
    ZPageY((u8, u8)),
    Relative(u8),
    AbsoluteIndirect(u16),
    AbsoluteIndexX((u16, u8)),
    AbsoluteIndexY((u16, u8)),
    Indirect(u16),
    IndirectX((u8, u8)),
    IndirectY((u8, u8)),
}

/// A `Cpu6502` is an abstraction for the 6502 emulator. 
/// 
/// MEMBERS: 
/// * `ram` is of type `EmuMemory`, which is split into three memory sections. These sections include
/// zero page, stack, and general purpose memory.
/// 
/// * `regs` is of type `Registers`, which defines the registers supported by the 6502 cpu.
#[derive(Debug)]
pub struct Cpu6502 {
    ram: EmuMemory,
    regs: Registers,
}

impl Cpu6502 {
    /// Initalization member function of the 6502 cpu.
    pub fn new() -> Self {
        Self {
            ram: EmuMemory::new(),
            regs: Registers::initalize(),
        }
    }
}

impl RomLoader for Cpu6502 {
    fn load_rom() {
        todo!()
    }
}
