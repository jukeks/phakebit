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
            let current_cycles = self.state.cycles;
            print!("{:X}", self.state.pc);
            let opcode = self.state.fetch_byte();
            let name = opcode_to_name(opcode);
            print!(" {:X} {:<8} ", opcode, name);

            match opcode {
                0x00 => break,
                0x01 => self.ora_xind(),
                0x05 => self.ora_zpg(),
                0x06 => self.asl_zpg(),
                0x08 => self.php_impl(),
                0x09 => self.ora_imm(),
                0x10 => self.bpl_rel(),
                0x18 => self.clc_impl(),
                0x20 => self.jsr_abs(),
                0x21 => self.and_xind(),
                0x2E => self.rol_abs(),
                0x38 => self.sec_impl(),
                0x48 => self.pha_impl(),
                0x4C => self.jmp_abs(),
                0x58 => self.cli_impl(),
                0x5D => self.eor_absx(),
                0x60 => self.rts_impl(),
                0x69 => self.adc_imm(),
                0x6D => self.adc_abs(),
                0x7D => self.adc_absx(),
                0x68 => self.pla_impl(),
                0x84 => self.sty_zpg(),
                0x85 => self.sta_zpg(),
                0x86 => self.stx_zpg(),
                0x88 => self.dey_impl(),
                0x8A => self.txa_impl(),
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
                0xE9 => self.sbc_imm(),
                0xED => self.sbc_abs(),
                0xEE => self.inc_abs(),
                _ => panic!("Unknown opcode: {:X}", opcode),
            }

            self.state.print_state();
            println!("{}", self.state.cycles - current_cycles);
        }
    }

    fn brk_impl(&mut self) {}

    fn clc_impl(&mut self) {
        self.state.status &= 0b1111_1110;
        self.state.cycles += 1;
    }

    fn bpl_rel(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        //  BPL uses relative addressing so it can branch to an address within -128 to +127 bytes
        let operand = unsigned_operand as i8;
        if self.state.get_n() == 0 {
            let pc = self.state.pc;
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            //println!("    jumping from 0x{:X} to 0x{:X}", pc, self.state.pc);
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

        let c = (value & 0b1000_0000) >> 7;
        let result = value << 1;
        self.state.write_byte(operand as u16, result);
        self.state.set_c(c);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
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
        //println!("    jumping to 0x{:X}", operand);
        let pc = self.state.pc;
        self.state.push_word(pc);
        self.state.set_pc(operand);
        self.state.increment_cycles(1);
    }

    fn rts_impl(&mut self) {
        let operand = self.state.pop_word();
        //println!("    return to 0x{:X}", operand);
        self.state.set_pc(operand);
        self.state.increment_cycles(1);
    }

    fn adc_imm(&mut self) {
        let operand = self.state.fetch_byte();
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;
        let sum = a as u16 + operand as u16 + carry as u16;
        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state.set_v(((a ^ result) & (operand ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn adc_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;
        let sum = a as u16 + value as u16 + carry as u16;
        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state.set_v(((a ^ result) & (value ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn adc_absx(&mut self) {
        let operand = self.state.fetch_word();
        let address = operand + self.state.x as u16;
        let value = self.state.read_byte(address);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;
        let sum = a as u16 + value as u16 + carry as u16;
        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state.set_v(((a ^ result) & (value ^ result) & 0x80) != 0);
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
        let old_carry = self.state.get_c() as u8;
    
        // Shift value left and bring in the old carry to bit 0
        let result = (value << 1) | old_carry;

        // New carry is the old bit 7
        let new_carry = (value & 0b1000_0000) >> 7;
    
        self.state.write_byte(operand, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.set_c(new_carry);
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

    fn txa_impl(&mut self) {
        self.state.a = self.state.x;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn dey_impl(&mut self) {
        self.state.y = self.state.y.wrapping_sub(1);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn stx_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.x);
        self.state.cycles += 1;
    }

    fn sta_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.a);
    }

    fn sty_abs(&mut self) {
        let operand = self.state.fetch_word();
        self.state.write_byte(operand, self.state.y);
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
        self.state.status &= 0b1111_1011;
        self.state.cycles += 1;
    }

    fn sta_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.write_byte(operand as u16, self.state.a);
    }

    fn stx_zpg(&mut self) {
        let operand = self.state.fetch_byte();
        self.state.write_byte(operand as u16, self.state.x);
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
        if self.state.get_c() == 1 {
            let pc = self.state.pc;
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            //println!("    jumping from 0x{:X} to 0x{:X}", pc, self.state.pc);
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
        self.state.set_c(if a >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn bne_rel(&mut self) {
        let unsigned_operand = self.state.fetch_byte();
        //  BNE uses relative addressing so it can branch to an address within -128 to +127 bytes
        let operand = unsigned_operand as i8;
        if self.state.get_z() == 0 {
            let pc = self.state.pc;
            self.state.set_pc(self.state.pc.wrapping_add(operand as u16));
            //println!("    jumping from 0x{:X} to 0x{:X}", pc, self.state.pc);
            self.state.increment_cycles(1);
        }
    }

    fn sbc_imm(&mut self) {
        let operand = self.state.fetch_byte();
        let value = self.state.get_a();
        let carry = self.state.get_c();
        
        // Perform the subtraction with inverted carry
        let result = value.wrapping_sub(operand).wrapping_sub(1 - carry);

        self.state.set_a(result);
        self.state.set_c(if value >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
        let overflow = ((value ^ result) & (operand ^ result) & 0x80) != 0;
        self.state.set_v(overflow);
        self.state.cycles += 1;
    }

    fn sbc_abs(&mut self) {
        let operand = self.state.fetch_word();
        let value = self.state.read_byte(operand);
        let a = self.state.get_a();
        let carry = self.state.get_c();
        
        // Perform the subtraction with inverted carry
        let result = a.wrapping_sub(value).wrapping_sub(1 - carry);

        self.state.set_a(result);
        self.state.set_c(if a >= value { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
        let overflow = ((a ^ result) & (value ^ result) & 0x80) != 0;
        self.state.set_v(overflow);
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
        let program = fs::read("./fixtures/day_of_week2.bin").expect("should be there");

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0x02000 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x20);

        let argument_addr: u16 = 0xDEAD;
        // 2008-05-01, should be thursday
        memory.set(argument_addr, 0x07);
        memory.set(argument_addr+1, 0xD8);
        memory.set(argument_addr+2, 0x05);
        memory.set(argument_addr+3, 0x01);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();
        
        cpu_state.x = 0xAD;
        cpu_state.y = 0xDE; 

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(10000);

        println!("cycles: {}", cpu.state.cycles);

        assert_eq!(cpu.state.a, 0x05);
    }
}



fn opcode_to_name(opcode: u8) -> &'static str {
    match opcode {
        0x00 => "BRK",
        0x01 => "ORA_XIND",
        0x05 => "ORA_ZPG",
        0x06 => "ASL_ZPG",
        0x08 => "PHP",
        0x09 => "ORA_IMM",
        0x10 => "BPL_REL",
        0x18 => "CLC",
        0x20 => "JSR_ABS",
        0x21 => "AND_XIND",
        0x2E => "ROL_ABS",
        0x38 => "SEC",
        0x48 => "PHA",
        0x4C => "JMP_ABS",
        0x58 => "CLI",
        0x5D => "EOR_ABSX",
        0x60 => "RTS",
        0x69 => "ADC_IMM",
        0x6D => "ADC_ABS",
        0x7D => "ADC_ABSX",
        0x68 => "PLA",
        0x84 => "STY_ZPG",
        0x86 => "STA_ZPG",
        0x88 => "DEY",
        0x8C => "STY_ABS",
        0x8D => "STA_ABS",
        0x8E => "STX_ABS",
        0x90 => "BCC",
        0x99 => "STA_ABSY",
        0xA0 => "LDY_IMM",
        0xA2 => "LDX_IMM",
        0xA4 => "LDY_ZPG",
        0xA6 => "LDX_ZPG",
        0xA8 => "TAY",
        0xA9 => "LDA_IMM",
        0xAA => "TAX",
        0xAC => "LDY_ABS",
        0xAD => "LDA_ABS",
        0xAE => "LDX_ABS",
        0xB0 => "BCS_REL",
        0xB1 => "LDA_YIND",
        0xCA => "DEX",
        0xC9 => "CMP_IMM",
        0xD0 => "BNE_REL",
        0xED => "SBC_ABS",
        0xEE => "INC_ABS",
        _ => "UNKNOWN",
    }
}
