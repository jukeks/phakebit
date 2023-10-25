pub trait Memory {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}

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
    fn get(&self, address: u16) -> u8 {
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
}
