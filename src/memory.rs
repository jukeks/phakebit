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
    use crate::cpu::CPU;

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
            read_count: u8,
        }

        impl Chip {
            fn new() -> Chip {
                Chip {
                    register: 0,
                    read_count: 0,
                }
            }
            /// A read changes the state of the chip. This is only possible
            /// because `Memory::get` takes `&mut self`.
            fn read(&mut self) -> u8 {
                self.read_count += 1;
                self.register
            }
            fn write(&mut self, value: u8) {
                self.register = value;
            }
        }

        struct MappedMemory {
            state: [u8; 0x10000],
            chip: Chip,
        }

        impl MappedMemory {
            fn new(chip: Chip) -> MappedMemory {
                MappedMemory {
                    state: [0; 0x10000],
                    chip,
                }
            }
        }

        impl Memory for MappedMemory {
            fn get(&mut self, address: u16) -> u8 {
                match address {
                    0x0000..=0x1FFF => self.state[address as usize],
                    0x2000..=0x3FFF => self.chip.read(),
                    0x4000..=0xFFFF => self.state[address as usize],
                }
            }

            fn set(&mut self, address: u16, value: u8) {
                match address {
                    0x0000..=0x1FFF => self.state[address as usize] = value,
                    0x2000..=0x3FFF => self.chip.write(value),
                    0x4000..=0xFFFF => self.state[address as usize] = value,
                }
            }
        }

        let mut m = MappedMemory::new(Chip::new());
        m.set(0x2000, 0x01);
        assert_eq!(m.get(0x2000), 0x01);
        assert_eq!(m.chip.read_count, 1);

        m.set(0x2000, 0x02);
        assert_eq!(m.get(0x2000), 0x02);
        assert_eq!(m.chip.read_count, 2);

        let cpu_state = crate::state::CPUState::new(m);
        let mut cpu = CPU::new(cpu_state);
        let state = cpu.get_mut_state();
        assert_eq!(state.read_byte(0x2000), 0x02);
    }
}
