pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const CARTRIDGE_RAM_BEGIN: usize = 0xA000;
pub const CARTRIDGE_RAM_END: usize = 0xBFFF;
pub const CARTRIDGE_RAM_SIZE: usize = CARTRIDGE_RAM_END - CARTRIDGE_RAM_BEGIN + 1;

pub const INTERNAL_RAM_BEGIN: usize = 0xC000;
pub const INTERNAL_RAM_END: usize = 0xDFFF;
pub const INTERNAL_RAM_SIZE: usize = INTERNAL_RAM_END - INTERNAL_RAM_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const UNUSED_BEGIN: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;

pub const IO_REGISTERS_BEGIN: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;

pub const ZERO_PAGE_BEGIN: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

pub const VBLANK_VECTOR: u16 = 0x40;
pub const LCDSTAT_VECTOR: u16 = 0x48;
pub const TIMER_VECTOR: u16 = 0x50;

pub struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    cartridge_ram: [u8; CARTRIDGE_RAM_SIZE],
    internal_ram: [u8; INTERNAL_RAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, game_rom: Vec<u8>) -> MemoryBus {
        let boot_rom = boot_rom_buffer.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!(
                    "Supplied boot ROM is the wrong size. Is {} bytes but should be {} bytes",
                    boot_rom_buffer.len(),
                    BOOT_ROM_SIZE
                );
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        for i in 0..ROM_BANK_0_SIZE {
            rom_bank_0[i] = game_rom[i];
        }

        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        for i in 0..ROM_BANK_N_SIZE {
            rom_bank_n[i] = game_rom[ROM_BANK_0_SIZE + i];
        }
        MemoryBus {
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            cartridge_ram: [0; CARTRIDGE_RAM_SIZE],
            internal_ram: [0; INTERNAL_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
        }
        
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_ROM_BEGIN..=BOOT_ROM_END =>
            {
                if let Some(boot_rom) = self.boot_rom {
                    boot_rom[address]
                }
                else {
                    self.rom_bank_0[address]
                }
            }
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => self.rom_bank_0[address],
            ROM_BANK_N_BEGIN..=ROM_BANK_N_END => self.rom_bank_n[address - ROM_BANK_N_BEGIN],
            //todo vram
            CARTRIDGE_RAM_BEGIN..=CARTRIDGE_RAM_END => self.cartridge_ram[address - CARTRIDGE_RAM_BEGIN],
            INTERNAL_RAM_BEGIN..=INTERNAL_RAM_END => self.internal_ram[address - INTERNAL_RAM_BEGIN],
            ECHO_RAM_BEGIN..=ECHO_RAM_END => self.internal_ram[address - ECHO_RAM_BEGIN],
            //todo oam
            UNUSED_BEGIN..=UNUSED_END => 0,
            ZERO_PAGE_BEGIN..=ZERO_PAGE_END => self.zero_page[address - ZERO_PAGE_BEGIN],
            //todo interupt flag
            _ => {
                panic!(
                    "Reading from an unkown part of memory at address 0x{:x}",
                    address
                );
            }
        }
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        match address {
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => {
                self.rom_bank_0[address] = byte;
            },
            VRAM_BEGIN..=VRAM_END => {
                //todo gpu
            },
            CARTRIDGE_RAM_BEGIN..=CARTRIDGE_RAM_END => {
                self.cartridge_ram[address - CARTRIDGE_RAM_BEGIN] = byte;
            },
            OAM_BEGIN..=OAM_END => {
                //todo more gpu
            },
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => {
                //todo io
            },
            UNUSED_BEGIN..=UNUSED_END => {/*DO NOTHING*/},
            ZERO_PAGE_BEGIN..=ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN] = byte;
            },
            INTERRUPT_ENABLE_REGISTER => {
                //todo interrupt reg
                panic!("interrupt not set up yet!");
            },
            _ => {
                panic!("Couldn't write to address 0x{:x} not a supported address", address);
            },
        }
    }
}

#[test]
fn test_write_cartridge_ram_begin() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = CARTRIDGE_RAM_BEGIN as u16;
    let expected = 0xAB;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}

#[test]
fn test_write_cartridge_ram_middle() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = CARTRIDGE_RAM_BEGIN as u16 + (CARTRIDGE_RAM_SIZE as u16 / 2);
    let expected = 0x34;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}

#[test]
fn test_write_cartridge_ram_end() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = CARTRIDGE_RAM_END as u16;
    let expected = 0xBC;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}

#[test]
fn test_write_rom_bank_0_begin() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = ROM_BANK_0_BEGIN as u16;
    let expected = 0x42;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}

#[test]
fn test_write_rom_bank_0_middle() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = (ROM_BANK_0_BEGIN + (ROM_BANK_0_SIZE / 2)) as u16;
    let expected = 0x44;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}

#[test]
fn test_write_rom_bank_0_end() {
    let game_rom: Vec<u8> = [0; ROM_BANK_0_SIZE + ROM_BANK_N_SIZE].to_vec();
    let mut memory_bus = MemoryBus::new(None, game_rom);
    let addr = ROM_BANK_0_END as u16;
    let expected = 0x45;
    memory_bus.write_byte(addr, expected);
    let value = memory_bus.read_byte(addr);
    assert_eq!(value, expected);
}