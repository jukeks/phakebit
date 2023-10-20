mod state;
mod cpu;
mod memory;

use state::CPUState;
use cpu::CPU;
use memory::Memory;

fn main() {
    let memory = Memory::new();
    let mut state = CPUState::new(memory);
    state.reset();
    let mut instruction_set = CPU::new(state);
    instruction_set.execute(1000);
}
