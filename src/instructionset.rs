use crate::cpu::CPU;

pub struct InstructionSet {
    cpu: CPU,
}

impl InstructionSet {
    pub fn new(cpu: CPU) -> InstructionSet {
        InstructionSet { cpu: cpu }
    }

    pub fn execute(&mut self, cycles: u64) {
        while self.cpu.cycles < cycles || cycles == 0 {
            let opcode = self.cpu.fetch_byte();
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
        let operand = self.cpu.fetch_byte();
        let address = self.cpu.read_x() as u16 + operand as u16;
        let value = self.cpu.read_byte(address);
        let a = self.cpu.get_a() | value;
        self.cpu.set_z(a);
        self.cpu.set_n(a);
    }

    fn ora_zpg(&mut self) {
        let operand = self.cpu.fetch_byte();
        let value = self.cpu.read_byte(operand as u16);
        let a = self.cpu.get_a() | value;
        self.cpu.set_a(a);
        self.cpu.set_z(a);
        self.cpu.set_n(a);
    }

    fn asl_zpg(&mut self) {
        let operand = self.cpu.fetch_byte();
        let value = self.cpu.read_byte(operand as u16);
        let c = value & 0b1000_0000;
        let result = value << 1;
        self.cpu.write_byte(operand as u16, result);
        self.cpu.set_z(result);
        self.cpu.set_n(result);
        self.cpu.set_c(c);
    }

    fn jmp_abs(&mut self) {
        let operand = self.cpu.fetch_word();
        self.cpu.set_pc(operand);
        self.cpu.increment_cycles(1);
    }

    fn php_impl(&mut self) {
        // set break and bit 5
        let status = self.cpu.status | 0b0011_0000;
        self.cpu.push_byte(status);
        self.cpu.cycles += 1;
    }

    fn ora_imm(&mut self) {
        let operand = self.cpu.fetch_byte();
        self.cpu.a |= operand;
        self.cpu.set_z(self.cpu.a);
        self.cpu.set_n(self.cpu.a);
        self.cpu.cycles += 1;
    }

    fn nop(&mut self) {
        self.cpu.cycles += 1;
    }

    fn cli_impl(&mut self) {
        self.cpu.status &= 0b1111_1101;
        self.cpu.cycles += 1;
    }

    fn ldx_imm(&mut self) {
        let operand = self.cpu.fetch_byte();
        self.cpu.x = operand;
        self.cpu.set_z(self.cpu.x);
        self.cpu.set_n(self.cpu.x);
        self.cpu.cycles += 1;
    }

    fn lda_imm(&mut self) {
        let operand = self.cpu.fetch_byte();
        self.cpu.a = operand;
        self.cpu.set_z(self.cpu.a);
        self.cpu.set_n(self.cpu.a);
        self.cpu.cycles += 1;
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

        let mut cpu = super::CPU::new(memory);
        cpu.reset();

        let mut is = super::InstructionSet::new(cpu);
        is.execute(1000);

        assert_eq!(is.cpu.a, 0xFF);
    }
}
