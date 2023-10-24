pub struct Memory {
    state: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            state: [0; 0x10000],
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        let idx = address as usize;
        self.state[idx]
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let idx = address as usize;
        self.state[idx] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn gets() {
        let mut m = Memory::new();
        let zero = m.get(0x0000);
        assert_eq!(zero, 0);

        m.set(0x1FF, 0x01);
        assert_eq!(m.get(0x1FF), 0x01);
    }
}
