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
            ArithmeticTarget::HLI => {
                let value = $self.bus.read_byte($self.registers.get_hl());
                $self.$work(value);
            },
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
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemoryBus::new(boot_rom, game_rom),
        }
    }
    fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::NOP => {
                (self.pc.wrapping_add(1), 4)
            },
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
            Instruction::AND(register) => {
                arithmetic_instruction!(register, self.and => a)
            },
            Instruction::OR(register) => {
                arithmetic_instruction!(register, self.or => a)
            },
            Instruction::XOR(register) => {
                arithmetic_instruction!(register, self.xor => a)
            },
            Instruction::CP(register) => {
                arithmetic_instruction!(register, self.compare)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                    };
                    match source {
                        LoadByteSource::D8 => (self.pc.wrapping_add(2), 8),
                        LoadByteSource::HLI => (self.pc.wrapping_add(1), 8),
                        _ => (self.pc.wrapping_add(1), 4),
                    }
                }
                LoadType::Word(target) => {
                    let word = self.read_next_word();
                    match target {
                        LoadWordTarget::BC => self.registers.set_bc(word),
                        LoadWordTarget::DE => self.registers.set_de(word),
                        LoadWordTarget::SP => self.sp = word,
                        LoadWordTarget::HL => self.registers.set_hl(word),
                    }
                    (self.pc.wrapping_add(3), 12)
                }
                LoadType::IndirectFromA(indirect) => {
                    let a = self.registers.a;
                    match indirect {
                        Indirect::BCIndirect => {
                            let mem_addr = self.registers.get_bc();
                            self.bus.write_byte(mem_addr, a)
                        },
                        Indirect::DEIndirect => {
                            let mem_addr = self.registers.get_de();
                            self.bus.write_byte(mem_addr, a)
                        },
                        Indirect::HLIndirectMinus => {
                            let mem_addr = self.registers.get_hl();
                            self.registers.set_hl(mem_addr.wrapping_sub(1));
                            self.bus.write_byte(mem_addr, a);
                        },
                        Indirect::HLIndirectPlus => {
                            let mem_addr = self.registers.get_hl();
                            self.registers.set_hl(mem_addr.wrapping_add(1));
                            self.bus.write_byte(mem_addr, a);
                        },
                        Indirect::WordIndirect => {
                            let mem_addr = self.read_next_word();
                            self.bus.write_byte(mem_addr, a)
                        },
                        Indirect::LastByteIndirect => {
                            let mem_addr = self.registers.c as u16 & 0xff00;
                            self.bus.write_byte(mem_addr, a)
                        }
                    }
                    match indirect {
                        Indirect::WordIndirect => (self.pc.wrapping_add(3), 16),
                        Indirect::LastByteIndirect => (self.pc.wrapping_add(2), 8),
                        _ => (self.pc.wrapping_add(1), 8),
                    }
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

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.clear();
        self.registers.f.zero = new_value == 0;
        self.registers.f.half_carry = true;
        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.clear();
        self.registers.f.zero = new_value == 0;
        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.clear();
        self.registers.f.zero = new_value == 0;
        new_value
    }

    fn compare(&mut self, value: u8) {
        self.registers.f.clear();
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        //Gameboy is little endian so the second byte as first half of the word
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc) + 1) as u16
    }
}
