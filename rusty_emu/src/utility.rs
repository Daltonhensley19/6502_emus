/// `RomLoader` is an ADT which extends `Cpu6502` to support rom management.
pub trait RomLoader {
    /// Associated function which enables the ability to load roms. 
    /// This feature is used to load test roms for 'Cpu6502'.
    fn load_rom();
}
