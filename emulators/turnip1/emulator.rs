use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use crate::{memory::MappedMemory, pia::PIAChip};
use phakebit::memory::Memory;
use phakebit::state::CPUState;
use phakebit::{cpu::CPU, state};

pub struct Emulator {
    kbd_rx: mpsc::Receiver<u8>,
    dsp_tx: mpsc::Sender<u8>,
}

impl Emulator {
    pub fn new(kbd_rx: mpsc::Receiver<u8>, dsp_tx: mpsc::Sender<u8>) -> Emulator {
        Emulator {
            kbd_rx: kbd_rx,
            dsp_tx: dsp_tx,
        }
    }

    pub fn execute_program(self, program: Vec<u8>, load_address: u16, start_address: u16) {
        let chip = Rc::new(RefCell::new(PIAChip::new(self.kbd_rx, self.dsp_tx)));

        let mut mem = MappedMemory::new(chip);

        for (i, byte) in program.iter().enumerate() {
            mem.set(load_address + i as u16, *byte);
        }

        let mut cpu_state = CPUState::new(mem);
        cpu_state.write_word(state::RESET_VECTOR_ADDR, start_address);
        cpu_state.reset();
        let mut cpu = CPU::new(cpu_state);

        const TARGET_TIME: u64 = 4000; // 4us per instruction or 1 MHz clock
        loop {
            let start = std::time::Instant::now();
            cpu.step();
            let end = std::time::Instant::now();
            let elapsed = end.duration_since(start).as_nanos() as u64;

            if elapsed < TARGET_TIME {
                let sleep_time = std::time::Duration::from_nanos(TARGET_TIME - elapsed);
                std::thread::sleep(sleep_time);
            }
        }
    }
}
