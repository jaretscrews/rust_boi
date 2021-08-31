#[derive(Debug)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    LD(LoadType),
    BIT(BitPosition),
}

#[derive(Debug)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    IndirectFromA(Indirect),
}

#[derive(Debug)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

#[derive(Debug)]
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
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8a => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8b => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8c => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8d => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8e => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0x8f => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9e => Some(Instruction::SBC(ArithmeticTarget::HLI)),
            0x9f => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xa0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xa1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xa2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xa3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xa4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xa5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xa6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xa7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xae => Some(Instruction::XOR(ArithmeticTarget::HLI)),
            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xb0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xb1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xb2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xb3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xb4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xb5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xb6 => Some(Instruction::OR(ArithmeticTarget::HLI)),
            0xb7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xb8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xba => Some(Instruction::CP(ArithmeticTarget::D)),
            0xbb => Some(Instruction::CP(ArithmeticTarget::E)),
            0xbc => Some(Instruction::CP(ArithmeticTarget::H)),
            0xbd => Some(Instruction::CP(ArithmeticTarget::L)),
            0xbe => Some(Instruction::CP(ArithmeticTarget::HLI)),
            0xbf => Some(Instruction::CP(ArithmeticTarget::A)),
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
