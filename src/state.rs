use crate::instruction::AddressingMode;
use crate::memory::Memory;

pub const STACK_PAGE: u16 = 0x100;
pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
pub const IRQ_VECTOR_ADDR: u16 = 0xFFFE;

pub struct CPUState {
    memory: Memory,

    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: u8,
    pub cycles: u64,
}

impl CPUState {
    pub fn new(memory: Memory) -> CPUState {
        CPUState {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            cycles: 0,
            memory: memory,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;
        self.status = 0x36;
        self.cycles = 0;

        self.pc = self.read_word(RESET_VECTOR_ADDR);
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let val = self.memory.get(address);
        val
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory.set(address, value);
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc += 1;
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let word = self.read_word(self.pc);
        self.pc += 2;
        word
    }

    pub fn push_byte(&mut self, value: u8) {
        self.write_byte(STACK_PAGE + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_word(&mut self, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        self.push_byte(high);
        self.push_byte(low);
    }

    pub fn pop_byte(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read_byte(0x100 + self.sp as u16)
    }

    pub fn pop_word(&mut self) -> u16 {
        let low = self.pop_byte() as u16;
        let high = self.pop_byte() as u16;
        (high << 8) | low
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn increment_cycles(&mut self, cycles: u64) {
        self.cycles += cycles;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_x(&mut self, value: u8) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: u8) {
        self.y = value;
    }

    pub fn set_n(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn set_z(&mut self, value: u8) {
        if value == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    pub fn set_c(&mut self, value: u8) {
        if value != 0 {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }
    }

    pub fn set_v(&mut self, v: bool) {
        if v {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }

    pub fn get_v(&self) -> u8 {
        (self.status & 0b0100_0000) >> 6
    }

    pub fn get_n(&self) -> u8 {
        (self.status & 0b1000_0000) >> 7
    }

    pub fn get_c(&self) -> u8 {
        self.status & 0b0000_0001
    }

    pub fn get_z(&self) -> u8 {
        (self.status & 0b0000_0010) >> 1
    }

    pub fn set_d(&mut self, value: u8) {
        if value != 0 {
            self.status |= 0b0000_1000;
        } else {
            self.status &= 0b1111_0111;
        }
    }

    pub fn get_d(&self) -> u8 {
        (self.status & 0b0000_1000) >> 3
    }

    pub fn resolve_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::ZPG => self.fetch_byte() as u16,
            AddressingMode::ZPGX => {
                let operand = self.fetch_byte();
                let address = operand.wrapping_add(self.x);
                address as u16
            }
            AddressingMode::ZPGY => {
                let operand = self.fetch_byte();
                let address = operand.wrapping_add(self.y);
                address as u16
            }
            AddressingMode::ABS => self.fetch_word(),
            AddressingMode::ABSX => {
                let operand = self.fetch_word();
                let address = operand.wrapping_add(self.x as u16);
                address
            }
            AddressingMode::ABSY => {
                let operand = self.fetch_word();
                let address = operand.wrapping_add(self.y as u16);
                address
            }
            AddressingMode::IND => {
                let indirect_address = self.fetch_word();
                self.read_word(indirect_address)
            }
            AddressingMode::XIND => {
                let operand = self.fetch_byte();
                // Wraps around to stay in zero-page
                let zero_page_address = (operand.wrapping_add(self.x)) as u8;
                let effective_address = self.read_word(zero_page_address as u16);
                effective_address
            }
            AddressingMode::INDY => {
                let zero_page_address = self.fetch_byte();
                let indirect_address = self.read_word(zero_page_address as u16);
                let effective_address = indirect_address.wrapping_add(self.y as u16);
                effective_address
            }
            AddressingMode::REL => {
                let unsigned_operand = self.fetch_byte();
                let operand = unsigned_operand as i8;
                self.pc.wrapping_add(operand as u16)
            }
            _ => panic!("Unsupported addressing mode: {:?}", mode),
        }
    }

    pub fn fetch_operand(&mut self, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::ACC => self.get_a(),
            AddressingMode::IMM => self.fetch_byte(),
            AddressingMode::ZPG => {
                let address = self.resolve_address(mode);
                self.read_byte(address as u16)
            }
            AddressingMode::ZPGX => {
                let address = self.resolve_address(mode);
                self.read_byte(address as u16)
            }
            AddressingMode::ZPGY => {
                let address = self.resolve_address(mode);
                self.read_byte(address as u16)
            }
            AddressingMode::ABS => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            AddressingMode::ABSX => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            AddressingMode::ABSY => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            AddressingMode::IND => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            AddressingMode::XIND => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            AddressingMode::INDY => {
                let address = self.resolve_address(mode);
                self.read_byte(address)
            }
            _ => panic!("Unsupported addressing mode: {:?}", mode),
        }
    }
}

mod tests {
    #[test]
    fn test_reset() {
        let mut cpu = super::CPUState::new(super::Memory::new());
        cpu.reset();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x0000);
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.status, 0x36);
    }

    #[test]
    fn test_write_word() {
        let memory = super::Memory::new();
        let mut cpu = super::CPUState::new(memory);
        cpu.reset();
        cpu.write_word(0x001, 0x1234);

        let value = cpu.read_word(0x0001);
        assert_eq!(value, 0x1234);
    }

    #[test]
    fn test_push_word() {
        let memory = super::Memory::new();
        let mut cpu = super::CPUState::new(memory);
        cpu.reset();
        cpu.push_word(0x1234);

        let value = cpu.pop_word();
        assert_eq!(value, 0x1234);
    }
}
