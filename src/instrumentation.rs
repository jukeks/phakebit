use crate::instruction;
use crate::instruction::AddressingMode;
use crate::instruction::Instruction;

pub struct Trace {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub sr: u8,
    pub instruction: Instruction,
    pub operand: Option<u16>,
}

impl Trace {
    pub fn new(
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        sr: u8,
        instruction: Instruction,
        operand: Option<u16>,
    ) -> Trace {
        Trace {
            pc: pc,
            a: a,
            x: x,
            y: y,
            sp: sp,
            sr: sr,
            instruction: instruction,
            operand: operand,
        }
    }

    pub fn print(&self) {
        let n_flag = (self.sr >> 7) & 1;
        let v_flag = (self.sr >> 6) & 1;
        let d_flag = (self.sr >> 3) & 1;
        let i_flag = (self.sr >> 2) & 1;
        let z_flag = (self.sr >> 1) & 1;
        let c_flag = self.sr & 1;

        let name: String = format_operation(self.instruction.operation);
        let operand: String = match self.operand {
            Some(operand) => format_operand(operand, self.pc, self.instruction.mode),
            None => "".to_string(),
        };

        let operand_asm: String = match self.operand {
            Some(operand) => {
                if operand > 0xFF {
                    let hi = (operand >> 8) as u8;
                    let lo = (operand & 0xFF) as u8;
                    format!("{:02X} {:02X}", lo, hi)
                } else {
                    format!("{:02X}   ", operand)
                }
            }
            None => "     ".to_string(),
        };

        println!(
            "{:04X} {:02X} {}  {} {:<9} |{:02X} {:02X} {:02X} {:02X}|{}{}{}{}{}{}|{} ",
            self.pc,
            self.instruction.opcode,
            operand_asm,
            name,
            operand,
            self.a,
            self.x,
            self.y,
            self.sp,
            n_flag,
            v_flag,
            d_flag,
            i_flag,
            z_flag,
            c_flag,
            self.instruction.cycles,
        );
    }
}

fn format_operation(operation: instruction::Operation) -> String {
    format!("{:?}", operation)
}

fn format_operand(operand: u16, pc: u16, mode: instruction::AddressingMode) -> String {
    match mode {
        AddressingMode::ACC => "A".to_string(),
        AddressingMode::ABS => format!("${:04X}", operand),
        AddressingMode::ABSX => format!("${:04X},X", operand),
        AddressingMode::ABSY => format!("${:04X},Y", operand),
        AddressingMode::IMM => format!("#${:02X}", operand),
        AddressingMode::IMPL => "".to_string(),
        AddressingMode::IND => format!("(${:04X})", operand),
        AddressingMode::XIND => format!("(${:02X},X)", operand),
        AddressingMode::INDY => format!("(${:02X}),Y", operand),
        AddressingMode::REL => {
            let signed_operand = operand as i8;
            format!(
                "${:04X}",
                // 2 comes from having read the opcode and operand bytes
                pc.wrapping_add(signed_operand as u16).wrapping_add(2)
            )
        }
        AddressingMode::ZPG => format!("${:02X}", operand),
        AddressingMode::ZPGX => format!("${:02X},X", operand),
        AddressingMode::ZPGY => format!("${:02X},Y", operand),
    }
}
