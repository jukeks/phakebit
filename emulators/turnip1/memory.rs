use std::{cell::RefCell, rc::Rc};

use phakebit::memory::Memory;

use crate::pia::PIAChip;

pub struct MappedMemory {
    state: [u8; 0x10000],
    chip: Rc<RefCell<PIAChip>>,
}

impl MappedMemory {
    pub fn new(chip: Rc<RefCell<PIAChip>>) -> MappedMemory {
        MappedMemory {
            state: [0; 0x10000],
            chip: chip,
        }
    }
}

impl Memory for MappedMemory {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x0000..=0xD00F => self.state[address as usize],
            0xD010..=0xD013 => (*self.chip).borrow_mut().read(address),
            0xD014..=0xFFFF => self.state[address as usize],
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0xD00F => self.state[address as usize] = value,
            0xD010..=0xD013 => (*self.chip).borrow_mut().write(address, value),
            0xD014..=0xFFFF => self.state[address as usize] = value,
        }
    }
}
