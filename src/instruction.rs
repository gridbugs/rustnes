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
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    ZeroPageXIndexed,
    ZeroPageYIndexed,
    ZeroPageIndirectXIndexed,
    ZeroPageIndirectYIndexed,
}

impl AddressingMode {
    fn decode0(_: u8) -> Result<Self> {
        unimplemented!()
    }

    fn decode1(bbb: u8) -> Result<Self> {
        match bbb {
            0 => Ok(AddressingMode::ZeroPageIndirectXIndexed),
            _ => unimplemented!(),
        }
    }

    fn decode2(_: u8) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum Instruction {
    // parametized instructions
    ORA(AddressingMode),
    AND(AddressingMode),
    EOR(AddressingMode),
    ADC(AddressingMode),
    STA(AddressingMode),
    LDA(AddressingMode),
    CMP(AddressingMode),
    SBC(AddressingMode),
    ASL(AddressingMode),
    ROL(AddressingMode),
    LSR(AddressingMode),
    ROR(AddressingMode),
    STX(AddressingMode),
    LDX(AddressingMode),
    DEC(AddressingMode),
    INC(AddressingMode),
    BIT(AddressingMode),
    STY(AddressingMode),
    LDY(AddressingMode),
    CPY(AddressingMode),
    CPX(AddressingMode),

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
    pub fn decode(encoded: u8) -> Result<Self> {
        // break encoding into sections by bits aaabbbcc
        let cc = encoded & 3;
        let bbb = encoded.wrapping_shr(2) & 7;
        let aaa = encoded.wrapping_shr(5);

        match cc {
            1 => match aaa {
                0 => Ok(Instruction::ORA(try!(AddressingMode::decode1(bbb)))),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
