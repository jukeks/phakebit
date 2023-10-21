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
    pub fn new(
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        sr: u8,
        opcode: u8,
        operand: Option<u16>,
        cycles: u8,
    ) -> Trace {
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
        let n_flag = (self.sr >> 7) & 1;
        let v_flag = (self.sr >> 6) & 1;
        let d_flag = (self.sr >> 3) & 1;
        let i_flag = (self.sr >> 2) & 1;
        let z_flag = (self.sr >> 1) & 1;
        let c_flag = self.sr & 1;

        let instruction = opcode_to_instruction(self.opcode);

        let name: String = format_operation(instruction.name);
        let operand: String = match self.operand {
            Some(operand) => format_operand(operand, self.pc, instruction.mode),
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
            self.opcode,
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
            self.cycles,
        );
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
enum AddressingMode {
    ACC,

    ABS,
    ABSX,
    ABSY,

    IMM,

    IMPL,

    IND,
    XIND,
    INDY,

    REL,

    ZPG,
    ZPGX,
    ZPGY,
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
enum Operation {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

struct Instruction {
    opcode: u8,
    name: Operation,
    mode: AddressingMode,
    cycles: u8,
}

fn format_operation(operation: Operation) -> String {
    format!("{:?}", operation)
}

fn format_operand(operand: u16, pc: u16, mode: AddressingMode) -> String {
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
                pc.wrapping_add(signed_operand as u16).wrapping_add(2)
            )
        }
        AddressingMode::ZPG => format!("${:02X}", operand),
        AddressingMode::ZPGX => format!("${:02X},X", operand),
        AddressingMode::ZPGY => format!("${:02X},Y", operand),
    }
}

fn opcode_to_instruction(opcode: u8) -> Instruction {
    match opcode {
        0x00 => Instruction {
            opcode: 0x00,
            name: Operation::BRK,
            mode: AddressingMode::IMPL,
            cycles: 7,
        },
        0x01 => Instruction {
            opcode: 0x01,
            name: Operation::ORA,
            mode: AddressingMode::XIND,
            cycles: 6,
        },
        0x05 => Instruction {
            opcode: 0x05,
            name: Operation::ORA,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0x06 => Instruction {
            opcode: 0x06,
            name: Operation::ASL,
            mode: AddressingMode::ZPG,
            cycles: 5,
        },
        0x08 => Instruction {
            opcode: 0x08,
            name: Operation::PHP,
            mode: AddressingMode::IMPL,
            cycles: 3,
        },
        0x09 => Instruction {
            opcode: 0x09,
            name: Operation::ORA,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0x10 => Instruction {
            opcode: 0x10,
            name: Operation::BPL,
            mode: AddressingMode::REL,
            cycles: 2,
        },
        0x18 => Instruction {
            opcode: 0x18,
            name: Operation::CLC,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0x20 => Instruction {
            opcode: 0x20,
            name: Operation::JSR,
            mode: AddressingMode::ABS,
            cycles: 6,
        },
        0x21 => Instruction {
            opcode: 0x21,
            name: Operation::AND,
            mode: AddressingMode::XIND,
            cycles: 6,
        },
        0x2E => Instruction {
            opcode: 0x2E,
            name: Operation::ROL,
            mode: AddressingMode::ABS,
            cycles: 6,
        },
        0x38 => Instruction {
            opcode: 0x38,
            name: Operation::SEC,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0x48 => Instruction {
            opcode: 0x48,
            name: Operation::PHA,
            mode: AddressingMode::IMPL,
            cycles: 3,
        },
        0x4C => Instruction {
            opcode: 0x4C,
            name: Operation::JMP,
            mode: AddressingMode::ABS,
            cycles: 3,
        },
        0x58 => Instruction {
            opcode: 0x58,
            name: Operation::CLI,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0x5D => Instruction {
            opcode: 0x5D,
            name: Operation::EOR,
            mode: AddressingMode::ABSX,
            cycles: 4,
        },
        0x60 => Instruction {
            opcode: 0x60,
            name: Operation::RTS,
            mode: AddressingMode::IMPL,
            cycles: 6,
        },
        0x69 => Instruction {
            opcode: 0x69,
            name: Operation::ADC,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0x6D => Instruction {
            opcode: 0x6D,
            name: Operation::ADC,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0x7D => Instruction {
            opcode: 0x7D,
            name: Operation::ADC,
            mode: AddressingMode::ABSX,
            cycles: 4,
        },
        0x68 => Instruction {
            opcode: 0x68,
            name: Operation::PLA,
            mode: AddressingMode::IMPL,
            cycles: 4,
        },
        0x84 => Instruction {
            opcode: 0x84,
            name: Operation::STY,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0x85 => Instruction {
            opcode: 0x85,
            name: Operation::STA,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0x86 => Instruction {
            opcode: 0x86,
            name: Operation::STX,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0x88 => Instruction {
            opcode: 0x88,
            name: Operation::DEY,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0x8C => Instruction {
            opcode: 0x8C,
            name: Operation::STY,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0x8D => Instruction {
            opcode: 0x8D,
            name: Operation::STA,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0x8E => Instruction {
            opcode: 0x8E,
            name: Operation::STX,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0x90 => Instruction {
            opcode: 0x90,
            name: Operation::BCC,
            mode: AddressingMode::REL,
            cycles: 2,
        },
        0x99 => Instruction {
            opcode: 0x99,
            name: Operation::STA,
            mode: AddressingMode::ABSY,
            cycles: 5,
        },
        0xA0 => Instruction {
            opcode: 0xA0,
            name: Operation::LDY,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0xA2 => Instruction {
            opcode: 0xA2,
            name: Operation::LDX,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0xA4 => Instruction {
            opcode: 0xA4,
            name: Operation::LDY,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0xA6 => Instruction {
            opcode: 0xA6,
            name: Operation::LDX,
            mode: AddressingMode::ZPG,
            cycles: 3,
        },
        0xA8 => Instruction {
            opcode: 0xA8,
            name: Operation::TAY,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0xA9 => Instruction {
            opcode: 0xA9,
            name: Operation::LDA,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0xAA => Instruction {
            opcode: 0xAA,
            name: Operation::TAX,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0xAC => Instruction {
            opcode: 0xAC,
            name: Operation::LDY,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0xAD => Instruction {
            opcode: 0xAD,
            name: Operation::LDA,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0xAE => Instruction {
            opcode: 0xAE,
            name: Operation::LDX,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0xB0 => Instruction {
            opcode: 0xB0,
            name: Operation::BCS,
            mode: AddressingMode::REL,
            cycles: 2,
        },
        0xB1 => Instruction {
            opcode: 0xB1,
            name: Operation::LDA,
            mode: AddressingMode::INDY,
            cycles: 5,
        },
        0xCA => Instruction {
            opcode: 0xCA,
            name: Operation::DEX,
            mode: AddressingMode::IMPL,
            cycles: 2,
        },
        0xC9 => Instruction {
            opcode: 0xC9,
            name: Operation::CMP,
            mode: AddressingMode::IMM,
            cycles: 2,
        },
        0xD0 => Instruction {
            opcode: 0xD0,
            name: Operation::BNE,
            mode: AddressingMode::REL,
            cycles: 2,
        },
        0xED => Instruction {
            opcode: 0xED,
            name: Operation::SBC,
            mode: AddressingMode::ABS,
            cycles: 4,
        },
        0xEE => Instruction {
            opcode: 0xEE,
            name: Operation::INC,
            mode: AddressingMode::ABS,
            cycles: 6,
        },
        _ => panic!("Unknown opcode: {:02X}", opcode),
    }
}
