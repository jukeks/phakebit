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
            let pc = self.state.pc;
            let opcode = self.state.fetch_byte();
            println!("read 0x{:X}, pc=0x{:X}", opcode, pc);
            match opcode {
                0x00 => break,
                0x01 => self.ora_xind(),
                0x05 => self.ora_zpg(),
                0x06 => self.asl_zpg(),
                0x08 => self.php_impl(),
                0x09 => self.ora_imm(),
                0x10 => self.bpl_rel(),
                0x18 => self.nop(),
                0x20 => self.jsr_abs(),
                0x21 => self.and_xind(),
                0x2E => self.rol_abs(),
                0x38 => self.sec_impl(),
                0x48 => self.pha_impl(),
                0x4C => self.jmp_abs(),
                0x5D => self.eor_absx(),
                0x60 => self.rts_impl(),
                0x69 => self.adc_imm(),
                0x6D => self.adc_abs(),
                0x7D => self.adc_absx(),
                0x68 => self.pla_impl(),
                0x84 => self.sty_zpg(),
                0x85 => self.cli_impl(),
                0x86 => self.sta_zpg(),
                0x88 => self.dey_impl(),
                0x8C => self.sty_abs(),
                0x8D => self.sta_abs(),
                0x8E => self.stx_abs(),
                0x90 => self.bcc_impl(),
                0x99 => self.sta_absy(),
                0xA0 => self.ldy_imm(),
                0xA2 => self.ldx_imm(),
                0xA4 => self.ldy_zpg(),
                0xA6 => self.ldx_zpg(),
                0xA8 => self.tay_impl(),
                0xA9 => self.lda_imm(),
                0xAA => self.tax_impl(),
                0xAC => self.ldy_abs(),
                0xAD => self.lda_abs(),
                0xAE => self.ldx_abs(),
                0xB0 => self.bcs_rel(),
                0xB1 => self.lda_yind(),
                0xCA => self.dex_impl(),
                0xC9 => self.cmp_imm(),
                0xD0 => self.bne_rel(),
                0xED => self.sbc_abs(),
                0xEE => self.inc_abs(),
                _ => panic!("Unknown opcode: {:X}", opcode),
            }
        }
    }

    fn brk_impl(&mut self) {}

    fn bpl_rel(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        //  BPL uses relative addressing so it can branch to an address within -128 to +127 bytes
        let operand = unsigned_operand as i8;
        println!("operand: {} vs unsigned_operand: {}", operand, unsigned_operand);

        if self.state.get_n() == 0 {
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            self.state.increment_cycles(1);
        }
    }

    fn ora_xind(&mut self) {
        let operand = self.state.fetch_byte();
        let address = self.state.read_x() as u16 + operand as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a() | value;
        self.state.set_a(a);
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

    fn eor_absx(&mut self) {
        let operand = self.state.fetch_word();
        let address = operand + self.state.x as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a() ^ value;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
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

    fn jsr_abs(&mut self) {
        let operand = self.state.fetch_word();
        println!("jumping to 0x{:X}", operand);
        let pc = self.state.pc;
        self.state.push_word(pc);
        self.state.set_pc(operand);
        self.state.increment_cycles(1);
    }

    fn rts_impl(&mut self) {
        let operand = self.state.pop_word();
        self.state.set_pc(operand);
        self.state.increment_cycles(1);
    }

    fn adc_imm(&mut self) {
        let operand = self.state.fetch_byte();
        let a = self.state.get_a();
        let result = a.wrapping_add(operand);
        self.state.set_a(result);
        self.state.set_v(a > 0xFF - operand);
        self.state.set_c(result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn adc_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let a = self.state.get_a();
        let result = a.wrapping_add(value);
        self.state.set_a(result);
        self.state.set_v(a > 0xFF - value);
        self.state.set_c(result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn adc_absx(&mut self) {
        let operand = self.state.fetch_word();
        let address = operand + self.state.x as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a();
        let result = a.wrapping_add(value);
        self.state.set_a(result);
        self.state.set_v(a > 0xFF - value);
        self.state.set_c(result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn pla_impl(&mut self) {
        let operand = self.state.pop_byte();
        self.state.a = operand;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn and_xind(&mut self) {
        let operand = self.state.fetch_byte();
        let address = self.state.read_x() as u16 + operand as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a() & value;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn rol_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let c = value & 0b1000_0000;
        let result = value << 1;
        self.state.write_byte(operand, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.set_c(c);
    }

    fn sec_impl(&mut self) {
        self.state.status |= 0b0000_0001;
        self.state.cycles += 1;
    }

    fn pha_impl(&mut self) {
        self.state.push_byte(self.state.a);
        self.state.cycles += 1;
    }

    fn sty_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.write_byte(operand as u16, self.state.y);
        self.state.cycles += 1;
    }

    fn dey_impl(&mut self) {
        self.state.y = self.state.y.wrapping_sub(1);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn stx_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.x);
        self.state.cycles += 1;
    }

    fn sta_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.a);
        self.state.cycles += 1;
    }

    fn sty_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.y);
        self.state.cycles += 1;
    }

    fn bcc_impl(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        let operand = unsigned_operand as i8;
        if self.state.get_c() == 0 {
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            self.state.increment_cycles(1);
        }
    }

    fn cli_impl(&mut self) {
        self.state.status &= 0b1111_1101;
        self.state.cycles += 1;
    }

    fn sta_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.write_byte(operand as u16, self.state.a);
        self.state.cycles += 1;
    }

    fn sta_absy(&mut self) {
        let operand = self.state.fetch_word();
        let address = operand + self.state.y as u16;
        self.state.write_byte(address, self.state.a);
        self.state.cycles += 1;
    }

    fn ldy_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.y = operand;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn ldx_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.x = operand;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn ldx_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        let value = self.state.read_byte(operand as u16);
        self.state.x = value;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn ldy_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        let value = self.state.read_byte(operand as u16);
        self.state.y = value;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn tax_impl(&mut self) {
        self.state.x = self.state.a;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn tay_impl(&mut self) {
        self.state.y = self.state.a;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn lda_imm(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.a = operand;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn lda_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        self.state.a = value;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn ldx_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        self.state.x = value;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn bcs_rel(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        //  BCS uses relative addressing so it can branch to an address within -128 to +127 bytes
        let operand = unsigned_operand as i8;
        println!("operand: {} vs unsigned_operand: {}", operand, unsigned_operand);

        if self.state.get_c() == 1 {
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            self.state.increment_cycles(1);
        }
    }

    fn ldy_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        self.state.y = value;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn lda_yind(&mut self) {
        let operand = self.state.fetch_byte();
        let address = self.state.read_word(operand as u16) + self.state.y as u16;
        let value = self.state.read_byte(address);
        self.state.a = value;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn dex_impl(&mut self) {
        self.state.x = self.state.x.wrapping_sub(1);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn cmp_imm(&mut self) {
        let operand = self.state.fetch_byte();
        let a = self.state.get_a();
        let result = a.wrapping_sub(operand);
        self.state.set_c(result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn bne_rel(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        //  BNE uses relative addressing so it can branch to an address within -128 to +127 bytes
        let operand = unsigned_operand as i8;
        println!("operand: {} vs unsigned_operand: {}", operand, unsigned_operand);

        if self.state.get_z() == 0 {
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            self.state.increment_cycles(1);
        }
    }

    fn sbc_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let a = self.state.get_a();
        let result = a.wrapping_sub(value);
        self.state.set_a(result);
        self.state.set_v(a < value);
        self.state.set_c(result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn inc_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let result = value.wrapping_add(1);
        self.state.write_byte(operand, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, state};
    use std::fs;

    #[test]
    fn test_simple_program() {
        let program: [u8; 11] = [
            0xA2, 0x00, 0xA9, 0x0F, 0x09, 0xF0, 0x85, 0x00, 0x4C, 0x08, 0x06,
        ];

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0x0600 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x06);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(1000);

        assert_eq!(cpu.state.a, 0xFF);
    }

    #[test]
    fn test_day_of_week_program() {
        let program = fs::read("./fixtures/day_of_week.bin").expect("should be there");

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0x02000 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x20);

        let argument_addr: u16 = 0xFF00;
        // 2008-05-01, should be thursday
        memory.set(argument_addr, 0x07);
        memory.set(argument_addr+1, 0xD8);
        memory.set(argument_addr+2, 0x05);
        memory.set(argument_addr+3, 0x01);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        cpu_state.x = 0x00;
        cpu_state.y = 0xFF;

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(10000);

        println!("cycles: {}", cpu.state.cycles);

        assert_eq!(cpu.state.a, 0x05);
    }

    #[test]
    fn test_instructions() {
        struct Setup {
            state: super::CPUState,
            writes: Vec<(u16, u8)>,
        }
        struct Instruction {
            opcode: u8,
            operand: Vec<u8>,
            setup: Setup,
            expected: super::CPUState,
        }
    }

    fn test_test_suite() {
        let program = fs::read("./fixtures/6502_functional_test.bin").expect("should be there");

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x04);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(10000);

        println!("cycles: {}", cpu.state.cycles);

        assert_eq!(cpu.state.a, 0x05);
    }
}
