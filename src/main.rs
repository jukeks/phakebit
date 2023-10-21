mod cpu;
mod memory;
mod state;
mod instrumentation;

use cpu::CPU;
use memory::Memory;
use state::CPUState;

fn main() {
    let memory = Memory::new();
    let mut state = CPUState::new(memory);
    state.reset();
    let mut instruction_set = CPU::new(state);
    instruction_set.execute(1000);
}
