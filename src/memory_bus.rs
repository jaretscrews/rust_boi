pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>) -> MemoryBus {
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
        
        MemoryBus {
            boot_rom
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
                    panic!("only should be doing boot rom");
                }
            }
            _ => {
                panic!(
                    "Reading from an unkown part of memory at address 0x{:x}",
                    address
                );
            }
        }
    }
}