mod cpu;
mod instruction;
mod instrumentation;
mod memory;
mod state;

use cpu::CPU;
use memory::Memory;
use state::CPUState;

fn main() {
    let memory = Memory::new();
    let mut state = CPUState::new(memory);
    state.reset();
    let mut cpu = CPU::new(state);
    cpu.execute(1000);
}
