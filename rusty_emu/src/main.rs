use rusty_emu::cpu::Cpu6502;

fn main() {
    let cpu = Cpu6502::new();

    println!("{cpu:?}");
}
