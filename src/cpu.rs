use crate::state::CPUState;

pub struct CPU {
    state: CPUState,
}

impl CPU {
    pub fn new(state: CPUState) -> CPU {
        CPU { state: state }
    }

    pub fn execute(&mut self, cycles: u64) {
        while self.state.cycles < cycles || cycles == 0 {
            let opcode = self.state.fetch_byte();
            match opcode {
                0x00 => self.brk_impl(),
                0x01 => self.ora_xind(),
                0x05 => self.ora_zpg(),
                0x06 => self.asl_zpg(),
                0x08 => self.php_impl(),
                0x09 => self.ora_imm(),
                0x18 => self.nop(),
                0x4C => self.jmp_abs(),
                0x85 => self.cli_impl(),
                0xA2 => self.ldx_imm(),
                0xA9 => self.lda_imm(),
                _ => panic!("Unknown opcode: {:X}", opcode),
            }
        }
    }

    fn brk_impl(&mut self) {}

    fn ora_xind(&mut self) {
        let operand = self.state.fetch_byte();
        let address = self.state.read_x() as u16 + operand as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a() | value;
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn ora_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        let value = self.state.read_byte(operand as u16);
        let a = self.state.get_a() | value;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn asl_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        let value = self.state.read_byte(operand as u16);
        let c = value & 0b1000_0000;
        let result = value << 1;
        self.state.write_byte(operand as u16, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.set_c(c);
    }

    fn jmp_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.set_pc(operand);
        self.state.increment_cycles(1);
    }

    fn php_impl(&mut self) {
        // set break and bit 5
        let status = self.state.status | 0b0011_0000;
        self.state.push_byte(status);
        self.state.cycles += 1;
    }

    fn ora_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.a |= operand;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn nop(&mut self) {
        self.state.cycles += 1;
    }

    fn cli_impl(&mut self) {
        self.state.status &= 0b1111_1101;
        self.state.cycles += 1;
    }

    fn ldx_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.x = operand;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn lda_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.a = operand;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::Memory;

    #[test]
    fn test_simple_program() {
        let program: [u8; 11] = [
            0xA2, 0x00, 0xA9, 0x0F, 0x09, 0xF0, 0x85, 0x00, 0x4C, 0x08, 0x06,
        ];

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0x0600 + i as u16, *byte);
        }

        memory.set(0xFFFC, 0x00);
        memory.set(0xFFFD, 0x06);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        let mut is = super::CPU::new(cpu_state);
        is.execute(1000);

        assert_eq!(is.state.a, 0xFF);
    }
}
