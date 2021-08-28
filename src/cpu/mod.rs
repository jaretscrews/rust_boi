pub mod flags_register;
pub mod instruction;
pub mod registers;

use crate::cpu::instruction::AritmeticTarget;
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

impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>, _game_rom: Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemoryBus::new(boot_rom),
        }
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => match target {
                AritmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                _ => self.pc,
            },
            Instruction::XOR(target) => match target {
                AritmeticTarget::A => {
                    let value = self.registers.a ^ self.registers.a;
                    self.registers.a = value;
                    self.registers.f.clear();
                    self.registers.f.zero = value == 0;

                    self.pc.wrapping_add(1)
                }
                _ => { panic!("todo more xors") }
            },
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => {
                            panic!("TODO: implement other sources")
                        }
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        _ => {
                            panic!("TODO: implement other targets")
                        }
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                },
                LoadType::Word(target) => {
                    let word = self.read_next_word();
                    match target {
                        LoadWordTarget::SP => self.sp = word,
                        LoadWordTarget::HL => self.registers.set_hl(word),
                        _ => {
                            panic!("TODO: impletent other load word targets")
                        }
                    }
                    self.pc.wrapping_add(3)
                },
                LoadType::IndirectFromA(indirect) => {
                    let a = self.registers.a;
                    match indirect {
                        Indirect::HLIndirectMinus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.bus.write_byte(hl, a);
                        }
                        _ => { panic!("todo more indirects") }
                    }
                    self.pc.wrapping_add(1)
                }
                _ => {
                    panic!("TODO: implement other load types")
                }
            },
            _ => {
                panic!("TODO: support more instructions")
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
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
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        //Gameboy is little endian so the second byte as first half of the word
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc) + 1) as u16
    }
}
