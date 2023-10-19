mod cpu;
mod instructionset;
mod memory;

use cpu::CPU;
use instructionset::InstructionSet;
use memory::Memory;

fn main() {
    let memory = Memory::new();
    let mut cpu = CPU::new(memory);
    cpu.reset();
    let mut instruction_set = InstructionSet::new(cpu);
    instruction_set.execute(1000);
}
