const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

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
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> Self {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(value: u8) -> Self {
        let zero = ((value >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((value >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((value >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((value >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn setup() -> Registers {
        Registers::new()
    }

    #[test]
    fn get_16_bit_registers() {
        let registers = setup();
        assert_eq!(registers.get_bc(), 0);
        assert_eq!(registers.get_de(), 0);
        assert_eq!(registers.get_hl(), 0);
    }

    #[test]
    fn set_16_bit_registers() {
        let mut registers = setup();
        registers.set_bc(0x00FF);
        registers.set_de(0xC0DE);
        registers.set_hl(0x0A0B);

        // sets 16 bit register pairs
        assert_eq!(registers.get_bc(), 0x00FF);
        assert_eq!(registers.get_de(), 0xC0DE);
        assert_eq!(registers.get_hl(), 0x0A0B);

        // sets the 8 bit registers individually
        assert_eq!(registers.b, 0);
        assert_eq!(registers.c, 0xFF);
        assert_eq!(registers.d, 0xC0);
        assert_eq!(registers.e, 0xDE);
        assert_eq!(registers.h, 0x0A);
        assert_eq!(registers.l, 0x0B);
    }
}
