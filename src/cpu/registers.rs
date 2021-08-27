use super::flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }
    
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0x00FF) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_af() {
        let mut registers = Registers::new();
        let test_af = 0b1010_1101_1111_0000;
        registers.set_af(test_af);
        assert_eq!(registers.a, 0b1010_1101u8);
        assert_eq!(registers.f, FlagsRegister::from(0b1111_0000u8));
        assert_eq!(test_af, registers.get_af());        
    }

    #[test]
    fn test_bc() {
        let mut registers = Registers::new();
        let test_bc = 0b1010_1111_1100_1100;
        registers.set_bc(test_bc);
        assert_eq!(registers.b, 0b1010_1111u8);
        assert_eq!(registers.c, 0b1100_1100u8);
        assert_eq!(test_bc, registers.get_bc());        
    }

    #[test]
    fn test_de() {
        let mut registers = Registers::new();
        let test_de = 0b1010_1110_1100_1101;
        registers.set_de(test_de);
        assert_eq!(registers.d, 0b1010_1110u8);
        assert_eq!(registers.e, 0b1100_1101u8);
        assert_eq!(test_de, registers.get_de());        
    }

    #[test]
    fn test_hl() {
        let mut registers = Registers::new();
        let test_hl = 0b0010_0111_1100_1100;
        registers.set_hl(test_hl);
        assert_eq!(registers.h, 0b0010_0111u8);
        assert_eq!(registers.l, 0b1100_1100u8);
        assert_eq!(test_hl, registers.get_hl());        
    }
}