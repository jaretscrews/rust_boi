pub enum Instruction {
    ADD(ArithmeticTarget),
    XOR(ArithmeticTarget),
    LD(LoadType),
    BIT(BitPosition),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    IndirectFromA(Indirect),
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

pub enum BitPosition {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x7c => Some(Instruction::BIT(BitPosition::B7)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectMinus))),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
