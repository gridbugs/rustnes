use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(u8),
}

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Memory(MemoryAddressingMode),
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
    STX(MemoryAddressingMode),
    LDX(MemoryAddressingMode),
    DEC(MemoryAddressingMode),
    INC(MemoryAddressingMode),
    BIT(MemoryAddressingMode),
    STY(MemoryAddressingMode),
    LDY(MemoryAddressingMode),
    CPY(MemoryAddressingMode),
    CPX(MemoryAddressingMode),

    // jumps
    JMP,  // absolute jump
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

use self::Instruction::*;
use self::AddressingMode::*;
use self::MemoryAddressingMode::*;

impl Instruction {
    pub fn decode(opcode: u8) -> Result<Self> {
        let instruction = match opcode {
            0x00 => BRK,
            0x01 => ORA(XIndexedIndirect),
            // 0x02
            // 0x03
            // 0x04
            0x05 => ORA(ZeroPage),
            0x06 => ASL(Memory(ZeroPage)),
            // 0x07
            0x08 => PHP,
            0x09 => ORA(Immediate),
            0x0a => ASL(Accumulator),
            // 0x0b
            // 0x0c
            0x0d => ORA(Absolute),
            0x0e => ASL(Memory(Absolute)),
            // 0x0f
            0x10 => BPL,
            0x11 => ORA(IndirectYIndexed),
            // 0x12
            // 0x13
            // 0x14
            0x15 => ORA(ZeroPageXIndexed),
            0x16 => ASL(Memory(ZeroPageXIndexed)),
            // 0x17
            0x18 => CLC,
            0x19 => ORA(AbsoluteYIndexed),
            // 0x1a
            // 0x1b
            // 0x1c
            0x1d => ORA(AbsoluteXIndexed),
            0x1e => ASL(Memory(AbsoluteXIndexed)),
            // 0x1f
            0x20 => JSR,
            0x21 => AND(XIndexedIndirect),
            // 0x22
            // 0x23
            0x24 => BIT(ZeroPage),
            0x25 => AND(ZeroPage),
            0x26 => ROL(Memory(ZeroPage)),
            // 0x27
            0x28 => PLP,
            0x29 => AND(Immediate),
            0x2a => ROL(Accumulator),
            // 0x2b
            0x2c => BIT(Absolute),
            0x2d => AND(Absolute),
            0x2e => ROL(Memory(Absolute)),
            // 0x2f
            0x30 => BMI,
            0x31 => AND(IndirectYIndexed),
            // 0x32
            // 0x33
            // 0x34
            0x35 => AND(ZeroPageXIndexed),
            0x36 => ROL(Memory(ZeroPageXIndexed)),
            // 0x37
            0x38 => SEC,
            0x39 => AND(AbsoluteYIndexed),
            // 0x3a
            // 0x3b
            // 0x3c
            0x3d => AND(AbsoluteXIndexed),
            0x3e => ROL(Memory(AbsoluteXIndexed)),
            // 0x3f
            0x40 => RTI,
            0x41 => EOR(XIndexedIndirect),
            // 0x42
            // 0x43
            // 0x44
            0x45 => EOR(ZeroPage),
            0x46 => LSR(Memory(ZeroPage)),
            // 0x47
            0x48 => PHA,
            0x49 => EOR(Immediate),
            0x4a => LSR(Accumulator),
            // 0x4b
            0x4c => JMP,
            0x4d => EOR(Absolute),
            0x4e => LSR(Memory(Absolute)),
            // 0x4f
            0x50 => BVC,
            0x51 => EOR(IndirectYIndexed),
            // 0x52
            // 0x53
            // 0x54
            0x55 => EOR(ZeroPage),
            0x56 => LSR(Memory(ZeroPage)),
            // 0x57
            0x58 => CLI,
            0x59 => EOR(AbsoluteYIndexed),
            // 0x5a
            // 0x5b
            // 0x5c
            0x5d => EOR(AbsoluteXIndexed),
            0x5e => LSR(Memory(AbsoluteXIndexed)),
            // 0x5f
            0x60 => RTS,
            0x61 => ADC(XIndexedIndirect),
            // 0x62
            // 0x63
            // 0x64
            0x65 => ADC(ZeroPage),
            0x66 => ROR(Memory(ZeroPage)),
            // 0x67
            0x68 => PLA,
            0x69 => ADC(Immediate),
            0x6a => ROR(Accumulator),
            // 0x6b
            0x6c => JMPI,
            0x6d => ADC(Absolute),
            0x6e => ROR(Memory(Absolute)),
            // 0x6f
            0x70 => BVS,
            0x71 => ADC(IndirectYIndexed),
            // 0x72
            // 0x73
            // 0x74
            0x75 => ADC(ZeroPageXIndexed),
            0x76 => ROR(Memory(ZeroPageXIndexed)),
            // 0x77
            0x78 => SEI,
            0x79 => ADC(AbsoluteYIndexed),
            // 0x7a
            // 0x7b
            // 0x7c
            0x7d => ADC(AbsoluteXIndexed),
            0x7e => ROR(Memory(AbsoluteXIndexed)),
            // 0x7f
            // 0x80
            0x81 => STA(XIndexedIndirect),
            // 0x82
            // 0x83
            0x84 => STY(ZeroPage),
            0x85 => STA(ZeroPage),
            0x86 => STX(ZeroPage),
            // 0x87
            0x88 => DEY,
            // 0x89
            0x8a => TXA,
            // 0x8b
            0x8c => STY(Absolute),
            0x8d => STA(Absolute),
            0x8e => STX(Absolute),
            // 0x8f
            0x90 => BCC,
            0x91 => STA(IndirectYIndexed),
            // 0x92
            // 0x93
            0x94 => STY(ZeroPageXIndexed),
            0x95 => STA(ZeroPageXIndexed),
            0x96 => STX(ZeroPageXIndexed),
            // 0x97
            0x98 => TYA,
            0x99 => STA(AbsoluteYIndexed),
            0x9a => TXS,
            // 0x9b
            // 0x9c
            0x9d => STA(AbsoluteXIndexed),
            // 0x9e
            // 0x9f
            0xa0 => LDY(Immediate),
            0xa1 => LDA(XIndexedIndirect),
            0xa2 => LDX(Immediate),
            // 0xa3
            0xa4 => LDY(ZeroPage),
            0xa5 => LDA(ZeroPage),
            0xa6 => LDX(ZeroPage),
            // 0xa7
            0xa8 => TAY,
            0xa9 => LDA(Immediate),
            0xaa => TAX,
            // 0xab
            0xac => LDY(Absolute),
            0xad => LDA(Absolute),
            0xae => LDX(Absolute),
            // 0xaf
            0xb0 => BCS,
            0xb1 => LDA(IndirectYIndexed),
            // 0xb2
            // 0xb3
            0xb4 => LDY(ZeroPageXIndexed),
            0xb5 => LDA(ZeroPageXIndexed),
            0xb6 => LDX(ZeroPageYIndexed),
            // 0xb7
            0xb8 => CLV,
            0xb9 => LDA(AbsoluteYIndexed),
            0xba => TSX,
            // 0xbb
            0xbc => LDY(AbsoluteXIndexed),
            0xbd => LDA(AbsoluteXIndexed),
            0xbe => LDX(AbsoluteYIndexed),
            // 0xbf
            0xc0 => CPY(Immediate),
            0xc1 => CMP(XIndexedIndirect),
            // 0xc2
            // 0xc3
            0xc4 => CPY(ZeroPage),
            0xc5 => CMP(ZeroPage),
            0xc6 => DEC(ZeroPage),
            // 0xc7
            0xc8 => INY,
            0xc9 => CMP(Immediate),
            0xca => DEX,
            // 0xcb
            0xcc => CPY(Absolute),
            0xcd => CMP(Absolute),
            0xce => DEC(Absolute),
            // 0xcf
            0xd0 => BNE,
            0xd1 => CMP(IndirectYIndexed),
            // 0xd2
            // 0xd3
            // 0xd4
            0xd5 => CMP(ZeroPageXIndexed),
            0xd6 => DEC(ZeroPageXIndexed),
            // 0xd7
            0xd8 => CLD,
            0xd9 => CMP(AbsoluteYIndexed),
            // 0xda
            // 0xdb
            // 0xdc
            0xdd => CMP(AbsoluteXIndexed),
            0xde => DEC(AbsoluteXIndexed),
            // 0xdf
            0xe0 => CPX(Immediate),
            0xe1 => SBC(XIndexedIndirect),
            // 0xe2
            // 0xe3
            0xe4 => CPX(ZeroPage),
            0xe5 => SBC(ZeroPage),
            0xe6 => INC(ZeroPage),
            // 0xe7
            0xe8 => INX,
            0xe9 => SBC(Immediate),
            0xea => NOP,
            // 0xeb
            0xec => CPX(Absolute),
            0xed => SBC(Absolute),
            0xee => INC(Absolute),
            // 0xef
            0xf0 => BEQ,
            0xf1 => SBC(IndirectYIndexed),
            // 0xf2
            // 0xf3
            // 0xf4
            0xf5 => SBC(ZeroPageXIndexed),
            0xf6 => INC(ZeroPageXIndexed),
            // 0xf7
            0xf8 => SED,
            0xf9 => SBC(AbsoluteYIndexed),
            // 0xfa
            // 0xfb
            // 0xfc
            0xfd => SBC(AbsoluteXIndexed),
            0xfe => INC(AbsoluteXIndexed),
            // 0xff
            _ => return Err(Error::InvalidOpcode(opcode)),
        };

        Ok(instruction)
    }
}
