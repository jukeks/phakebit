//! Implementation of the instruction set

use crate::instruction;
use crate::instruction::AddressingMode;
use crate::instruction::Operation;
use crate::instrumentation::Trace;
use crate::memory::Memory;
use crate::state::CPUState;
use crate::state::IRQ_VECTOR_ADDR;

/// The CPU emulator
pub struct CPU<T: Memory> {
    state: CPUState<T>,
}

/// Implementation of the instruction set.
/// Instructions are executed against the `CPUState` struct.
impl<T: Memory> CPU<T> {
    pub fn new(state: CPUState<T>) -> CPU<T> {
        CPU { state: state }
    }

    pub fn get_mut_state(&mut self) -> &mut CPUState<T> {
        &mut self.state
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

    /// Execute the CPU for a given number of cycles.
    /// Prints a trace of each instruction executed to stdout.
    pub fn execute(&mut self, cycles: u64) {
        while self.state.cycles < cycles || cycles == 0 {
            let trace = self.step();
            trace.print();
        }
    }

    /// Execute the next instruction.
    pub fn step(&mut self) -> Trace {
        let pc = self.state.pc;
        let opcode = self.state.fetch_byte();
        let instruction = instruction::opcode_to_instruction(opcode);
        let operand = self.read_operand(instruction.mode);

        match instruction.operation {
            Operation::BRK => self.brk(),
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

        self.state.increment_cycles(instruction.cycles as u64);

        Trace::new(
            pc,
            self.state.a,
            self.state.x,
            self.state.y,
            self.state.sp,
            self.state.status,
            instruction,
            operand,
        )
    }

    fn brk(&mut self) {
        self.state.pc += 1;
        let return_address = self.state.pc;
        self.state.push_word(return_address);
        let mut status = self.state.status;
        status |= 0b0011_0000;
        self.state.push_byte(status);
        self.state.pc = self.state.read_word(IRQ_VECTOR_ADDR);
        self.state.status |= 0b0000_0100; // set interrupt disable
    }

    fn adc(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;

        let sum = if self.state.get_d() == 1 {
            let mut result = a as u16 + operand as u16 + carry as u16;
            if (a & 0x0F) + (operand & 0x0F) + carry > 0x09 {
                result += 0x06;
            }
            if result > 0x99 {
                result += 0x60;
            }
            result
        } else {
            a as u16 + operand as u16 + carry as u16
        };

        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state
            .set_v(((a ^ result) & (operand ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
    }

    fn sbc(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let carry = self.state.get_c() as u8;

        let sum = if self.state.get_d() == 1 {
            let mut result = a as u16 + (!operand) as u16 + carry as u16;
            if (a & 0x0F) < (operand & 0x0F) + 1 - carry {
                result -= 0x06;
            }
            if result < 0x100 {
                result -= 0x60;
            }
            result
        } else {
            a as u16 + (!operand) as u16 + carry as u16
        };

        self.state.set_c((sum > 0xFF) as u8);
        let result = (sum & 0xFF) as u8;
        self.state.set_a(result);
        self.state
            .set_v(((a ^ result) & (!operand ^ result) & 0x80) != 0);
        self.state.set_z(result);
        self.state.set_n(result);
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.set_x(operand);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.set_y(operand);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn lda(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        self.state.set_a(operand);
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
    }

    fn ora(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() | operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn asl(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::ACC => {
                let c = (self.state.a & 0b1000_0000) >> 7;
                let result = self.state.a << 1;
                self.state.set_a(result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
            _ => {
                let address = self.state.resolve_address(mode);
                let value = self.state.read_byte(address);
                let c = (value & 0b1000_0000) >> 7;
                let result = value << 1;
                self.state.write_byte(address, result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
        }
    }

    fn sta(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.a);
    }

    fn stx(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.x);
    }

    fn sty(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.write_byte(address, self.state.y);
    }

    fn jmp(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn dey(&mut self) {
        self.state.set_y(self.state.y.wrapping_sub(1));
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn dex(&mut self) {
        self.state.set_x(self.state.x.wrapping_sub(1));
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn iny(&mut self) {
        self.state.set_y(self.state.y.wrapping_add(1));
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn inx(&mut self) {
        self.state.set_x(self.state.x.wrapping_add(1));
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
    }

    fn php(&mut self) {
        // set break and bit 5
        let status = self.state.status | 0b0011_0000;
        self.state.push_byte(status);
    }

    fn cmp(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let result = a.wrapping_sub(operand);
        self.state.set_c(if a >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
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
        self.state.set_a(self.state.x);
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
    }

    fn tya(&mut self) {
        self.state.set_a(self.state.y);
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
    }

    fn txs(&mut self) {
        self.state.sp = self.state.x;
    }

    fn tay(&mut self) {
        self.state.set_y(self.state.a);
        self.state.set_z(self.state.y);
        self.state.set_n(self.state.y);
    }

    fn tax(&mut self) {
        self.state.set_x(self.state.a);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn clc(&mut self) {
        self.state.status &= 0b1111_1110;
    }

    fn sec(&mut self) {
        self.state.status |= 0b0000_0001;
    }

    fn cli(&mut self) {
        self.state.status &= 0b1111_1011;
    }

    fn sei(&mut self) {
        self.state.status |= 0b0000_0100;
    }

    fn clv(&mut self) {
        self.state.status &= 0b1011_1111;
    }

    fn cld(&mut self) {
        self.state.status &= 0b1111_0111;
    }

    fn sed(&mut self) {
        self.state.set_d(1);
    }

    fn jsr(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let return_address = self.state.pc - 1;
        self.state.push_word(return_address);
        self.state.set_pc(address);
    }

    fn rts(&mut self) {
        let return_address = self.state.pop_word();
        self.state.set_pc(return_address + 1);
    }

    fn rol(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::ACC => {
                let c = (self.state.a & 0b1000_0000) >> 7;
                let result = (self.state.a << 1) | self.state.get_c();
                self.state.set_a(result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
            _ => {
                let address = self.state.resolve_address(mode);
                let value = self.state.read_byte(address);
                let c = (value & 0b1000_0000) >> 7;
                let result = (value << 1) | self.state.get_c();
                self.state.write_byte(address, result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
        }
    }

    fn bne(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        if self.state.get_z() == 0 {
            self.state.set_pc(address);
        }
    }

    fn pla(&mut self) {
        let value = self.state.pop_byte();
        self.state.set_a(value);
        self.state.set_z(self.state.a);
        self.state.set_n(self.state.a);
    }

    fn plp(&mut self) {
        self.state.status = self.state.pop_byte() & 0b1110_1111; // ignore break flag
    }

    fn and(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() & operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn eor(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a() ^ operand;
        self.state.set_a(a);
        self.state.set_z(a);
        self.state.set_n(a);
    }

    fn lsr(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::ACC => {
                let c = self.state.a & 0b0000_0001;
                let result = self.state.a >> 1;
                self.state.set_a(result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
            _ => {
                let address = self.state.resolve_address(mode);
                let value = self.state.read_byte(address);
                let c = value & 0b0000_0001;
                let result = value >> 1;
                self.state.write_byte(address, result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
        }
    }

    fn ror(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::ACC => {
                let c = self.state.a & 0b0000_0001;
                let result = (self.state.a >> 1) | (self.state.get_c() << 7);
                self.state.set_a(result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
            _ => {
                let address = self.state.resolve_address(mode);
                let value = self.state.read_byte(address);
                let c = value & 0b0000_0001;
                let result = (value >> 1) | (self.state.get_c() << 7);
                self.state.write_byte(address, result);
                self.state.set_c(c);
                self.state.set_z(result);
                self.state.set_n(result);
            }
        }
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
        self.state.status = self.state.pop_byte() & 0b1110_1111; // ignore break flag
        let address = self.state.pop_word();
        self.state.set_pc(address);
        self.state.increment_cycles(1);
    }

    fn nop(&mut self) {}

    fn inc(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let result = value.wrapping_add(1);
        self.state.write_byte(address, result);
        self.state.set_z(result);
        self.state.set_n(result);
    }

    fn dec(&mut self, mode: AddressingMode) {
        let address = self.state.resolve_address(mode);
        let value = self.state.read_byte(address);
        let result = value.wrapping_sub(1);
        self.state.write_byte(address, result);
        self.state.set_z(result);
        self.state.set_n(result);
    }

    fn tsx(&mut self) {
        self.state.set_x(self.state.sp);
        self.state.set_z(self.state.x);
        self.state.set_n(self.state.x);
    }

    fn bit(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let a = self.state.get_a();
        let result = a & operand;
        self.state.set_z(result);
        self.state.set_n(operand);
        self.state.set_v((operand & 0b0100_0000) != 0);
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
    }

    fn cpy(&mut self, mode: AddressingMode) {
        let operand = self.state.fetch_operand(mode);
        let y = self.state.get_y();
        let result = y.wrapping_sub(operand);
        self.state.set_c(if y >= operand { 1 } else { 0 });
        self.state.set_z(result);
        self.state.set_n(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::{instrumentation::Trace, memory::Memory, memory::PlainMemory, state};
    use circular_buffer::CircularBuffer;
    use std::fs;

    #[test]
    fn test_simple_program() {
        let program: [u8; 11] = [
            0xA2, 0x00, 0xA9, 0x0F, 0x09, 0xF0, 0x85, 0x00, 0x4C, 0x08, 0x06,
        ];

        let mut memory = PlainMemory::new();
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
    fn run_test_suite() {
        // https://github.com/Klaus2m5/6502_65C02_functional_tests/blob/7954e2dbb49c469ea286070bf46cdd71aeb29e4b/bin_files/6502_functional_test.lst
        let program = fs::read("./fixtures/6502_functional_test.bin").expect("should be there");

        let mut memory = PlainMemory::new();
        for (i, byte) in program.iter().enumerate() {
            memory.set(0x0000 + i as u16, *byte);
        }

        memory.set(state::RESET_VECTOR_ADDR, 0x00);
        memory.set(state::RESET_VECTOR_ADDR + 1, 0x04);

        let mut cpu_state = super::CPUState::new(memory);
        cpu_state.reset();

        let mut cpu = super::CPU::new(cpu_state);

        let mut buffer = CircularBuffer::<100, Trace>::new();
        let mut stuck = false;
        loop {
            let trace = cpu.step();
            if buffer.back().is_some() {
                let last_pc = buffer.back().unwrap().pc;
                stuck = trace.pc == last_pc;
            }

            buffer.push_back(trace);
            if stuck {
                break;
            }
        }

        if stuck {
            for trace in buffer.iter() {
                trace.print();
            }
        }

        // 0x3469 is the last instruction in the test suite
        if cpu.state.pc != 0x3469 {
            assert!(!stuck);
        }
    }
}
