pub mod flags_register;
pub mod instruction;
pub mod registers;

use crate::cpu::instruction::ArithmeticTarget;
use crate::cpu::instruction::Instruction;
use crate::cpu::registers::Registers;
use crate::memory_bus::MemoryBus;

use self::instruction::*;

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

macro_rules! manipulate_8bit_register {
    ($self:ident  : $getter:ident => $work:ident) => {
        $self.$work($self.registers.$getter)
    };
    ($self:ident  : $getter:ident => $work:ident, $result_register:ident) => {{
        let value = manipulate_8bit_register!($self: $getter => $work);
        $self.registers.$result_register = value;
    }};
}

#[macro_export]
macro_rules! arithmetic_instruction {
    ($register:ident, $self:ident.$work:ident) => {{
        match $register {
            ArithmeticTarget::A => manipulate_8bit_register!($self: a => $work),
            ArithmeticTarget::B => manipulate_8bit_register!($self: b => $work),
            ArithmeticTarget::C => manipulate_8bit_register!($self: c => $work),
            ArithmeticTarget::D => manipulate_8bit_register!($self: d => $work),
            ArithmeticTarget::E => manipulate_8bit_register!($self: e => $work),
            ArithmeticTarget::H => manipulate_8bit_register!($self: h => $work),
            ArithmeticTarget::L => manipulate_8bit_register!($self: l => $work),
        };

        match $register {
            _ => {($self.pc.wrapping_add(1), 1)}
        }
    }};
    ($register:ident, $self:ident.$work:ident => $result_register:ident) => {{
        match $register {
            ArithmeticTarget::A => {manipulate_8bit_register!($self: a => $work, $result_register)},
            ArithmeticTarget::B => {manipulate_8bit_register!($self: b => $work, $result_register)},
            ArithmeticTarget::C => {manipulate_8bit_register!($self: c => $work, $result_register)},
            ArithmeticTarget::D => {manipulate_8bit_register!($self: d => $work, $result_register)},
            ArithmeticTarget::E => {manipulate_8bit_register!($self: e => $work, $result_register)},
            ArithmeticTarget::H => {manipulate_8bit_register!($self: h => $work, $result_register)},
            ArithmeticTarget::L => {manipulate_8bit_register!($self: l => $work, $result_register)},
            ArithmeticTarget::HLI => {
                let value = $self.bus.read_byte($self.registers.get_hl());
                let result = $self.$work(value);
                $self.registers.a = result;
            },
        };

        match $register {
            ArithmeticTarget::HLI => ($self.pc.wrapping_add(1), 8),
            _ => {($self.pc.wrapping_add(1), 4)}
        }
    }};
}

impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>, _game_rom: Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemoryBus::new(boot_rom),
        }
    }
    fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::ADD(register) => {
                arithmetic_instruction!(register, self.add_without_carry => a)
            },
            Instruction::ADC(register) => {
                arithmetic_instruction!(register, self.add_with_carry => a)
            },
            Instruction::SUB(register) => {
                arithmetic_instruction!(register, self.sub_without_carry => a)
            },
            Instruction::SBC(register) => {
                arithmetic_instruction!(register, self.sub_with_carry => a)
            },
            Instruction::XOR(register) => {
                arithmetic_instruction!(register, self.xor => a)
            },
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => {
                            panic!("TODO: implement other sources {:?}", source)
                        }
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        _ => {
                            panic!("TODO: implement other targets {:?}", target)
                        }
                    };
                    match source {
                        LoadByteSource::D8 => (self.pc.wrapping_add(2), 1),
                        _ => (self.pc.wrapping_add(1), 1),
                    }
                }
                LoadType::Word(target) => {
                    let word = self.read_next_word();
                    match target {
                        LoadWordTarget::SP => self.sp = word,
                        LoadWordTarget::HL => self.registers.set_hl(word),
                        _ => {
                            panic!("TODO: impletent other load word targets {:?}", target)
                        }
                    }
                    (self.pc.wrapping_add(3), 1)
                }
                LoadType::IndirectFromA(indirect) => {
                    let a = self.registers.a;
                    match indirect {
                        Indirect::HLIndirectMinus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.bus.write_byte(hl, a);
                        }
                        _ => {
                            panic!("todo more indirects {:?}", indirect)
                        }
                    }
                    (self.pc.wrapping_add(1), 1)
                }
                _ => {
                    panic!("TODO: implement other load types {:?}", load_type)
                }
            },

            _ => {
                panic!("TODO: support more instructions {:?}", instruction)
            }
        }
    }

    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let (next_pc, cycles) = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description)
        };

        self.pc = next_pc;
        cycles
    }
    fn sub_with_carry(&mut self, value: u8) -> u8 {
        self.sub(value, true)
    }

    fn sub_without_carry(&mut self, value: u8) -> u8 {
        self.sub(value, false)
    }

    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let carry_value:u8 = (carry && self.registers.f.carry) as u8;

        let (new_value, overflow) = self.registers.a.overflowing_sub(value);
        let (new_value2, overflow2) = new_value.overflowing_sub(carry_value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = overflow || overflow2;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + carry_value;
        new_value2
    }

    fn add_without_carry(&mut self, value: u8) -> u8 {
        self.add(value, false)
    }

    fn add_with_carry(&mut self, value: u8) -> u8 {
        self.add(value, true)
    }

    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let carry_value:u8 = (carry && self.registers.f.carry) as u8;
        let (new_value, overflow) = self.registers.a.overflowing_add(value);
        let (new_value2, overflow2) = new_value.overflowing_add(carry_value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow || overflow2;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + carry_value > 0xF;

        new_value2
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.clear();
        self.registers.f.zero = value == 0;
        new_value
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        //Gameboy is little endian so the second byte as first half of the word
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc) + 1) as u16
    }
}
