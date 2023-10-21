
pub struct Trace {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    sr: u8,
    opcode: u8,
    operand: Option<u16>,
    cycles: u8,
}

impl Trace {
    pub fn new(pc: u16, a: u8, x: u8, y: u8, sp: u8, sr: u8, opcode: u8, operand: Option<u16>, cycles: u8) -> Trace {
        Trace {
            pc: pc,
            a: a,
            x: x,
            y: y,
            sp: sp,
            sr: sr,
            cycles: cycles,
            opcode: opcode,
            operand: operand,
        }
    }

    pub fn print(&self) {
        let n_flag = (self.sp >> 7) & 1;
        let v_flag = (self.sp >> 6) & 1;
        let d_flag = (self.sp >> 3) & 1;
        let i_flag = (self.sp >> 2) & 1;
        let z_flag = (self.sp >> 1) & 1;
        let c_flag = self.sp & 1;
        
        let name = opcode_to_name(self.opcode);

        let operand: String = match self.operand {
            Some(operand) => {
                if operand > 0xFF {
                    let hi = (operand >> 8) as u8;
                    let lo = (operand & 0xFF) as u8;
                    format!("{:02X} {:02X}", lo, hi)
                } else {
                    format!("{:02X}   ", operand)
                }
            },
            None => "     ".to_string(),
        };

        println!("{:04X} {:02X} {}  {:<8} |{:02X} {:02X} {:02X} {:02X}|{}{}{}{}{}{}|{} ", self.pc, self.opcode, operand, name, self.a, self.x, self.y, self.sp, n_flag, v_flag, d_flag, i_flag, z_flag, c_flag ,self.cycles, );
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
