//! Models the memory

/// Abstract memory interface
pub trait Memory {
    fn get(&mut self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}

/// Plain memory implementation with just 64K of RAM
pub struct PlainMemory {
    state: [u8; 0x10000],
}

impl PlainMemory {
    pub fn new() -> PlainMemory {
        PlainMemory {
            state: [0; 0x10000],
        }
    }
}

impl Memory for PlainMemory {
    fn get(&mut self, address: u16) -> u8 {
        let idx = address as usize;
        self.state[idx]
    }

    fn set(&mut self, address: u16, value: u8) {
        let idx = address as usize;
        self.state[idx] = value;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Memory;
    use super::PlainMemory;

    #[test]
    fn gets() {
        let mut m = PlainMemory::new();
        let zero = m.get(0x0000);
        assert_eq!(zero, 0);

        m.set(0x1FF, 0x01);
        assert_eq!(m.get(0x1FF), 0x01);
    }

    #[test]
    fn memory_maps() {
        struct Chip {
            register: u8,
        }

        impl Chip {
            fn new() -> Chip {
                Chip { register: 0 }
            }
            fn read(&self) -> u8 {
                self.register
            }
            fn write(&mut self, value: u8) {
                self.register = value;
            }
        }

        struct MappedMemory {
            state: [u8; 0x10000],
            chip: Rc<RefCell<Chip>>,
        }

        impl MappedMemory {
            fn new(chip: Rc<RefCell<Chip>>) -> MappedMemory {
                MappedMemory {
                    state: [0; 0x10000],
                    chip: chip,
                }
            }
        }

        impl Memory for MappedMemory {
            fn get(&mut self, address: u16) -> u8 {
                match address {
                    0x0000..=0x1FFF => self.state[address as usize],
                    0x2000..=0x3FFF => self.chip.borrow().read(),
                    0x4000..=0xFFFF => self.state[address as usize],
                }
            }

            fn set(&mut self, address: u16, value: u8) {
                match address {
                    0x0000..=0x1FFF => self.state[address as usize] = value,
                    0x2000..=0x3FFF => (*self.chip).borrow_mut().write(value),
                    0x4000..=0xFFFF => self.state[address as usize] = value,
                }
            }
        }

        let chip = Rc::new(RefCell::new(Chip::new()));
        let mut m = MappedMemory::new(chip.clone());
        m.set(0x2000, 0x01);
        assert_eq!(chip.borrow().read(), 0x01);
        assert_eq!(m.get(0x2000), 0x01);

        (*chip).borrow_mut().write(0x02);
        assert_eq!(m.get(0x2000), 0x02);
        assert_eq!(chip.borrow().read(), 0x02);
    }
}
