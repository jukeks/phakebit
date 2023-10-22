use crate::instruction;
use crate::instruction::AddressingMode;
use crate::instruction::Operation;
use crate::instrumentation::Trace;
use crate::state::CPUState;

pub struct CPU {
    state: CPUState,
}

impl CPU {
    pub fn new(state: CPUState) -> CPU {
        CPU { state: state }
    }

    fn read_operand(&mut self, mode: AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::REL => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::ACC => None,
            AddressingMode::IMPL => None,
            AddressingMode::IMM => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::ZPG => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::ZPGX => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::ZPGY => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::ABS => Some(self.state.read_word(self.state.pc)),
            AddressingMode::ABSX => Some(self.state.read_word(self.state.pc)),
            AddressingMode::ABSY => Some(self.state.read_word(self.state.pc)),
            AddressingMode::IND => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::XIND => Some(self.state.read_byte(self.state.pc) as u16),
            AddressingMode::INDY => Some(self.state.read_byte(self.state.pc) as u16),
        }
    }

    pub fn execute(&mut self, cycles: u64) {
        while self.state.cycles < cycles || cycles == 0 {
            let pc = self.state.pc;
            let current_cycles = self.state.cycles;
            let opcode = self.state.fetch_byte();
            let instruction = instruction::opcode_to_instruction(opcode);
            let operand = self.read_operand(instruction.mode);

            match instruction.name {
                Operation::BRK => break,
                Operation::ADC => self.adc(instruction.mode),
                Operation::LDX => self.ldx(instruction.mode),
                Operation::LDA => self.lda(instruction.mode),
                Operation::LDY => self.ldy(instruction.mode),
                Operation::ASL => self.asl(instruction.mode),
                Operation::ORA => self.ora(instruction.mode),
                Operation::STA => self.sta(instruction.mode),
                Operation::STX => self.stx(instruction.mode),
                Operation::STY => self.sty(instruction.mode),
                Operation::JMP => self.jmp(instruction.mode),
                Operation::DEY => self.dey(),
                Operation::DEX => self.dex(),
                Operation::INY => self.iny(),
                Operation::INX => self.inx(),
                Operation::BPL => self.bpl(instruction.mode),
                Operation::PHA => self.pha(),
                Operation::PHP => self.php(),
                Operation::CMP => self.cmp(instruction.mode),
                Operation::BCS => self.bcs(instruction.mode),
                Operation::BCC => self.bcc(instruction.mode),
                Operation::TXA => self.txa(),
                Operation::TYA => self.tya(),
                Operation::TXS => self.txs(),
                Operation::TSX => self.tsx(),
                Operation::TAY => self.tay(),
                Operation::TAX => self.tax(),
                Operation::CLC => self.clc(),
                Operation::SEC => self.sec(),
                Operation::CLI => self.cli(),
                Operation::SEI => self.sei(),
                Operation::CLV => self.clv(),
                Operation::CLD => self.cld(),
                Operation::SED => self.sed(),
                Operation::SBC => self.sbc(instruction.mode),
                Operation::JSR => self.jsr(instruction.mode),
                Operation::RTS => self.rts(),
                Operation::ROL => self.rol(instruction.mode),
                Operation::BNE => self.bne(instruction.mode),
                Operation::PLA => self.pla(),
                Operation::PLP => self.plp(),
                Operation::AND => self.and(instruction.mode),
                Operation::EOR => self.eor(instruction.mode),
                Operation::LSR => self.lsr(instruction.mode),
                Operation::ROR => self.ror(instruction.mode),
                Operation::BMI => self.bmi(instruction.mode),
                Operation::BVS => self.bvs(instruction.mode),
                Operation::BVC => self.bvc(instruction.mode),
                Operation::RTI => self.rti(),
                Operation::NOP => self.nop(),
                Operation::BEQ => self.beq(instruction.mode),
                Operation::CPX => self.cpx(instruction.mode),
                Operation::CPY => self.cpy(instruction.mode),
                Operation::INC => self.inc(instruction.mode),
                Operation::DEC => self.dec(instruction.mode),
                Operation::BIT => self.bit(instruction.mode),
            }

            let t = Trace::new(
                pc,
                self.state.a,
                self.state.x,
                self.state.y,
                self.state.sp,
                self.state.status,
                opcode,
                operand,
                0,
            );
            t.print();
        }
    }

    fn adc(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;
        let sum = a as u16 + operand as u16 + carry as u16;
        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state
            .set_v(((a ^ result) & (operand ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.x = operand;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.y = operand;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn lda(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.a = operand;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn ora(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() | operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
        self.state.cycles += 1;
    }

    fn asl(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let c = (value & 0b1000_0000) >> 7;
        let result = value << 1;
        self.state.write_byte(address, result);
        self.state.set_c(c);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn sta(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.a);
        self.state.cycles += 1;
    }

    fn stx(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.x);
        self.state.cycles += 1;
    }

    fn sty(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.y);
        self.state.cycles += 1;
    }

    fn jmp(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn dey(&mut self) {
        self.state.y = self.state.y.wrapping_sub(1);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn dex(&mut self) {
        self.state.x = self.state.x.wrapping_sub(1);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn iny(&mut self) {
        self.state.y = self.state.y.wrapping_add(1);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn inx(&mut self) {
        self.state.x = self.state.x.wrapping_add(1);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn bpl(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_n() == 0 {
            self.state.set_pc(address);
            self.state.increment_cycles(1);
        }
    }

    fn pha(&mut self) {
        self.state.push_byte(self.state.a);
        self.state.cycles += 1;
    }

    fn php(&mut self) {
        // set break and bit 5
        let status = self.state.status | 0b0011_0000;
        self.state.push_byte(status);
        self.state.cycles += 1;
    }

    fn cmp(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let result = a.wrapping_sub(operand);
        self.state.set_c(if a >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn bcs(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_c() == 1 {
            self.state.set_pc(address);
        }
    }

    fn bcc(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_c() == 0 {
            self.state.set_pc(address);
        }
    }

    fn txa(&mut self) {
        self.state.a = self.state.x;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn tya(&mut self) {
        self.state.a = self.state.y;
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn txs(&mut self) {
        self.state.sp = self.state.x;
        self.state.cycles += 1;
    }

    fn tay(&mut self) {
        self.state.y = self.state.a;
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
        self.state.cycles += 1;
    }

    fn tax(&mut self) {
        self.state.x = self.state.a;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn clc(&mut self) {
        self.state.status &= 0b1111_1110;
        self.state.cycles += 1;
    }

    fn sec(&mut self) {
        self.state.status |= 0b0000_0001;
        self.state.cycles += 1;
    }

    fn cli(&mut self) {
        self.state.status &= 0b1111_1101;
        self.state.cycles += 1;
    }

    fn sei(&mut self) {
        self.state.status |= 0b0000_0100;
        self.state.cycles += 1;
    }

    fn clv(&mut self) {
        self.state.status &= 0b1011_1111;
        self.state.cycles += 1;
    }

    fn cld(&mut self) {
        self.state.status &= 0b1111_0111;
        self.state.cycles += 1;
    }

    fn sed(&mut self) {
        self.state.status |= 0b0000_1000;
        self.state.cycles += 1;
        panic!("Decimal mode not implemented");
    }

    fn sbc(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;
        let sum = a as u16 + (!operand) as u16 + carry as u16;
        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state
            .set_v(((a ^ result) & (!operand ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn jsr(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let pc = self.state.pc;
        self.state.push_word(pc);
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn rts(&mut self) {
        let address = self.state.pop_word();
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn rol(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let c = (value & 0b1000_0000) >> 7;
        let result = (value << 1) | self.state.get_c();
        self.state.write_byte(address, result);
        self.state.set_c(c);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn bne(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_z() == 0 {
            self.state.set_pc(address);
        }
    }

    fn pla(&mut self) {
        self.state.a = self.state.pop_byte();
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
        self.state.cycles += 1;
    }

    fn plp(&mut self) {
        self.state.status = self.state.pop_byte();
        self.state.cycles += 1;
    }

    fn and(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() & operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
        self.state.cycles += 1;
    }

    fn eor(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() ^ operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
        self.state.cycles += 1;
    }

    fn lsr(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let c = value & 0b0000_0001;
        let result = value >> 1;
        self.state.write_byte(address, result);
        self.state.set_c(c);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn ror(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let c = value & 0b0000_0001;
        let result = (value >> 1) | (self.state.get_c() << 7);
        self.state.write_byte(address, result);
        self.state.set_c(c);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn bmi(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_n() == 1 {
            self.state.set_pc(address);
        }
    }

    fn bvs(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_v() == 1 {
            self.state.set_pc(address);
        }
    }

    fn bvc(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_v() == 0 {
            self.state.set_pc(address);
        }
    }

    fn rti(&mut self) {
        self.state.status = self.state.pop_byte();
        let address = self.state.pop_word();
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn nop(&mut self) {
        self.state.cycles += 1;
    }

    fn inc(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let result = value.wrapping_add(1);
        self.state.write_byte(address, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn dec(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let result = value.wrapping_sub(1);
        self.state.write_byte(address, result);
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn tsx(&mut self) {
        self.state.x = self.state.sp;
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
        self.state.cycles += 1;
    }

    fn bit(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let result = a & operand;
        self.state.set_z(result);
        self.state.set_n(operand);
        self.state.set_v((operand & 0b0100_0000) != 0);
        self.state.cycles += 1;
    }

    fn beq(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_z() == 1 {
            self.state.set_pc(address);
        }
    }

    fn cpx(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let x = self.state.get_x();
        let result = x.wrapping_sub(operand);
        self.state.set_c(if x >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
        self.state.cycles += 1;
    }

    fn cpy(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let y = self.state.get_y();
        let result = y.wrapping_sub(operand);
        self.state.set_c(if y >= operand { 1 } else { 0 });
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
        memory.set(argument_addr + 1, 0xD8);
        memory.set(argument_addr + 2, 0x05);
        memory.set(argument_addr + 3, 0x01);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        cpu_state.x = 0xAD;
        cpu_state.y = 0xDE;

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(10000);

        println!("cycles: {}", cpu.state.cycles);

        //assert_eq!(cpu.state.a, 0x05); // 0x05 is thursday but it's not working
        assert_eq!(cpu.state.a, 0x01);
    }

    #[test]
    fn run_test_suite() {
        let program = fs::read("./fixtures/6502_functional_test.bin").expect("should be there");

        let mut memory = Memory::new();
        for (i, byte) in program.iter().enumerate() {
            if i > 0x4000 {
                break;
            }
            memory.set(0x0000 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x04);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        let mut cpu = super::CPU::new(cpu_state);
        cpu.execute(1000000);

        println!("cycles: {}", cpu.state.cycles);
        assert!(cpu.state.cycles > 100000000)
    }
}
