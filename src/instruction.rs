use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidAddressingModeForInstruction,
    InvalidOpcode,
}

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Memory(MemoryAddressingMode),
}

impl AddressingMode {
    fn decode_cc2(bbb: u8) -> Result<Self> {
        match bbb {
            0 => Ok(AddressingMode::Memory(MemoryAddressingMode::Immediate)),
            1 => Ok(AddressingMode::Memory(MemoryAddressingMode::ZeroPage)),
            2 => Ok(AddressingMode::Accumulator),
            3 => Ok(AddressingMode::Memory(MemoryAddressingMode::Absolute)),
            5 => Ok(AddressingMode::Memory(MemoryAddressingMode::ZeroPageXIndexed)),
            7 => Ok(AddressingMode::Memory(MemoryAddressingMode::AbsoluteXIndexed)),
            _ => Err(Error::InvalidAddressingModeForInstruction),
        }
    }
}

// Addressing modes that access memory
#[derive(Debug)]
pub enum MemoryAddressingMode {
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    ZeroPageXIndexed,
    ZeroPageYIndexed,
    XIndexedIndirect,
    IndirectYIndexed,
}

impl MemoryAddressingMode {
    fn decode_cc0(bbb: u8) -> Result<Self> {
        match bbb {
            0 => Ok(MemoryAddressingMode::Immediate),
            1 => Ok(MemoryAddressingMode::ZeroPage),
            3 => Ok(MemoryAddressingMode::Absolute),
            5 => Ok(MemoryAddressingMode::ZeroPageXIndexed),
            7 => Ok(MemoryAddressingMode::AbsoluteXIndexed),
            _ => Err(Error::InvalidAddressingModeForInstruction),
        }
    }

    fn decode_cc1(bbb: u8) -> Self {
        match bbb {
            0 => MemoryAddressingMode::XIndexedIndirect,
            1 => MemoryAddressingMode::ZeroPage,
            2 => MemoryAddressingMode::Immediate,
            3 => MemoryAddressingMode::Absolute,
            4 => MemoryAddressingMode::IndirectYIndexed,
            5 => MemoryAddressingMode::ZeroPageXIndexed,
            6 => MemoryAddressingMode::AbsoluteYIndexed,
            7 => MemoryAddressingMode::AbsoluteXIndexed,
            _ => panic!("Incorrect mask in instruction decoding."),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    // parametized instructions
    ORA(MemoryAddressingMode),
    AND(MemoryAddressingMode),
    EOR(MemoryAddressingMode),
    ADC(MemoryAddressingMode),
    STA(MemoryAddressingMode),
    LDA(MemoryAddressingMode),
    CMP(MemoryAddressingMode),
    SBC(MemoryAddressingMode),
    ASL(AddressingMode),
    ROL(AddressingMode),
    LSR(AddressingMode),
    ROR(AddressingMode),
    STX(AddressingMode),
    LDX(AddressingMode),
    DEC(AddressingMode),
    INC(AddressingMode),
    BIT(MemoryAddressingMode),
    STY(MemoryAddressingMode),
    LDY(MemoryAddressingMode),
    CPY(MemoryAddressingMode),
    CPX(MemoryAddressingMode),

    // jumps
    JMP,
    JMPI, // indirect jump

    // conditional branches
    BPL,
    BMI,
    BVC,
    BVS,
    BCC,
    BCS,
    BNE,
    BEQ,

    // interrupts and subroutines
    BRK,
    JSR,
    RTI,
    RTS,

    // others
    PHP,
    PLP,
    PHA,
    PLA,
    DEY,
    TAY,
    INY,
    INX,
    CLC,
    SEC,
    CLI,
    SEI,
    TYA,
    CLV,
    CLD,
    SED,
    TXA,
    TXS,
    TAX,
    TSX,
    DEX,
    NOP,
}

impl Instruction {
    fn decode_branch(aaa: u8) -> Self {
        match aaa {
            0 => Instruction::BPL,
            1 => Instruction::BMI,
            2 => Instruction::BVC,
            3 => Instruction::BVS,
            4 => Instruction::BCC,
            5 => Instruction::BCS,
            6 => Instruction::BNE,
            7 => Instruction::BEQ,
            _ => panic!("Incorrect mask in instruction decoding."),
        }
    }

    fn decode_iqr_or_function_instruction(aaa: u8) -> Option<Self> {
        let instruction = match aaa {
            0 => Instruction::BRK,
            1 => Instruction::JSR,
            2 => Instruction::RTI,
            3 => Instruction::RTS,
            _ => return None
        };

        Some(instruction)
    }

    fn decode_extra_inter_register_instructions_bbb2(aaa: u8) -> Option<Self> {
        let instruction = match aaa {
            4 => Instruction::TXA,
            5 => Instruction::TAX,
            6 => Instruction::DEX,
            _ => return None,
        };

        Some(instruction)
    }

    fn decode_extra_inter_register_instructions_bbb6(aaa: u8) -> Option<Self> {
        let instruction = match aaa {
            4 => Instruction::TXS,
            5 => Instruction::TSX,
            6 => Instruction::NOP,
            _ => return None,
        };

        Some(instruction)
    }

    fn decode_inter_register_instruction(aaa: u8) -> Self {
        match aaa {
            0 => Instruction::PHP,
            1 => Instruction::PLP,
            2 => Instruction::PHA,
            3 => Instruction::PLA,
            4 => Instruction::DEY,
            5 => Instruction::TAY,
            6 => Instruction::INY,
            7 => Instruction::INX,
            _ => panic!("Incorrect mask in instruction decoding."),
        }
    }

    fn decode_status_register_instruction(aaa: u8) -> Self {
        match aaa {
            0 => Instruction::CLC,
            1 => Instruction::SEC,
            2 => Instruction::CLI,
            3 => Instruction::SEI,
            4 => Instruction::TYA,
            5 => Instruction::CLV,
            6 => Instruction::CLD,
            7 => Instruction::SED,
            _ => panic!("Incorrect mask in instruction decoding."),
        }
    }

    pub fn decode(encoded: u8) -> Result<Self> {
        // break encoding into sections by bits aaabbbcc
        let cc = encoded & 3;
        let bbb = encoded.wrapping_shr(2) & 7;
        let aaa = encoded.wrapping_shr(5);

        match cc {
            0 => {
                if let Some(instruction) = match bbb {
                    0 => Self::decode_iqr_or_function_instruction(aaa),
                    _ => None,
                } {
                    return Ok(instruction);
                }

                let instruction = match bbb {
                    2 => Self::decode_inter_register_instruction(aaa),
                    4 => Self::decode_branch(aaa),
                    6 => Self::decode_status_register_instruction(aaa),
                    other => match aaa {
                        0 => {
                            unimplemented!()
                        },
                        1 => Instruction::BIT(try!(MemoryAddressingMode::decode_cc0(other))),
                        2 => Instruction::JMP,
                        3 => Instruction::JMPI,
                        4 => Instruction::STY(try!(MemoryAddressingMode::decode_cc0(other))),
                        5 => Instruction::LDY(try!(MemoryAddressingMode::decode_cc0(other))),
                        6 => Instruction::CPY(try!(MemoryAddressingMode::decode_cc0(other))),
                        7 => Instruction::CPX(try!(MemoryAddressingMode::decode_cc0(other))),
                        _ => panic!("Incorrect mask in instruction decoding."),
                    }
                };
                Ok(instruction)
            },
            1 => {
                let instruction = match aaa {
                    0 => Instruction::ORA(MemoryAddressingMode::decode_cc1(bbb)),
                    1 => Instruction::AND(MemoryAddressingMode::decode_cc1(bbb)),
                    2 => Instruction::EOR(MemoryAddressingMode::decode_cc1(bbb)),
                    3 => Instruction::ADC(MemoryAddressingMode::decode_cc1(bbb)),
                    4 => Instruction::STA(MemoryAddressingMode::decode_cc1(bbb)),
                    5 => Instruction::LDA(MemoryAddressingMode::decode_cc1(bbb)),
                    6 => Instruction::CMP(MemoryAddressingMode::decode_cc1(bbb)),
                    7 => Instruction::SBC(MemoryAddressingMode::decode_cc1(bbb)),
                    _ => panic!("Incorrect mask in instruction decoding."),
                };
                Ok(instruction)
            },
            2 => {
                if let Some(instruction) = match bbb {
                    2 => Self::decode_extra_inter_register_instructions_bbb2(aaa),
                    6 => Self::decode_extra_inter_register_instructions_bbb6(aaa),
                    _ => None,
                } {
                    return Ok(instruction);
                }

                let instruction = match aaa {
                    0 => Instruction::ASL(try!(AddressingMode::decode_cc2(bbb))),
                    1 => Instruction::ROL(try!(AddressingMode::decode_cc2(bbb))),
                    2 => Instruction::LSR(try!(AddressingMode::decode_cc2(bbb))),
                    3 => Instruction::ROR(try!(AddressingMode::decode_cc2(bbb))),
                    4 => Instruction::STX(try!(AddressingMode::decode_cc2(bbb))),
                    5 => Instruction::LDX(try!(AddressingMode::decode_cc2(bbb))),
                    6 => Instruction::DEC(try!(AddressingMode::decode_cc2(bbb))),
                    7 => Instruction::INC(try!(AddressingMode::decode_cc2(bbb))),
                    _ => panic!("Incorrect mask in instruction decoding."),
                };
                Ok(instruction)
            },
            _ => Err(Error::InvalidOpcode),
        }
    }
}
