#![allow(unused)]

/// Newtype pattern to provide a type safe abstraction for a signed 8-bit word.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Word(i8);

#[derive(Debug)]
struct MemoryRegion<const SIZE: usize> {
    buffer: [Word; SIZE],
}

const Z_PAGE_SIZE: usize = 0x00FF - 0x0000;
const STACK_SIZE: usize = 0x01FF - 0x0100;
const GENERAL_MEM_SIZE: usize = 0xFFFF - 0x0200;

/// `EmuMemory` is an abstraction for the sections of memory which is accessed by the 6502
/// emulator.
#[derive(Debug)]
pub struct EmuMemory {
    /// The first page of memory, which is faster to access than other pages.
    /// Instructions can specify addresses within the zero page with a single byte as opposed to two, so
    /// instructions that use the zero page instead of any other page require one less CPU cycle to
    /// execute.
    zero_page: MemoryRegion<Z_PAGE_SIZE>,
    /// Last-in first-out data structure. Grows backwards from $01FF to $0100.
    /// Used by some transfer, stack, and subroutine instructions.
    stack: MemoryRegion<STACK_SIZE>,
    /// Memory that can be used for whatever purpose needed.
    /// Devices that use the 6502 processor may choose to reserve sub-regions for other purposes,
    /// such as memory-mapped I/O.
    gen_purpose: MemoryRegion<GENERAL_MEM_SIZE>,
}

impl EmuMemory {
    /// Initalization of the three memory regions: `zero_page`, `stack`, and `gen_purpose`. 
    pub fn new() -> Self {
        Self {
            zero_page: MemoryRegion {
                buffer: [Word(0); Z_PAGE_SIZE],
            },
            stack: MemoryRegion {
                buffer: [Word(0); STACK_SIZE],
            },
            gen_purpose: MemoryRegion {
                buffer: [Word(0); GENERAL_MEM_SIZE],
            },
        }
    }
}
