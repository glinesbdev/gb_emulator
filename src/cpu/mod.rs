mod instructions;
mod registers;

use crate::memory::*;
use instructions::*;
use registers::Registers;

/// # CPU Macros
///
/// These macros make it easier to read/write from/to either 8 or 16 bit registers to cut down on repeated code.
///
/// # 8 bit register macros
macro_rules! operate_8bit_register {
    // Operate on an 8 bit register and return the value
    //
    // The following macro pattern breaks down like this:
    // operate_8bit_register!(a => self.fn)
    ($getter:ident => $self:ident.$fn:ident) => {
        {
            $self.$fn($self.registers.$getter)
        }
    };

    // Set the result of an 8 bit register operation into another register
    //
    // The following macro pattern breaks down like this:
    // operate_8bit_register!(b => self.fn => a)
    ($getter:ident => $self:ident.$fn:ident => $setter:ident) => {
        {
            $self.registers.$setter = operate_8bit_register!($getter => $self.$fn);
        }
    };

    ($getter:ident => $self:ident.$fn:ident @ $arg:expr) => {
        {
            $self.$fn($self.registers.$getter, $arg);
        }
    };

    ($getter:ident => ($self:ident.$fn:ident @ $arg:expr) => $setter:ident) => {
        {
            $self.registers.$setter = $self.$fn($self.registers.$getter, $arg);
        }
    };
}

macro_rules! operate_16bit_register {
    // Get the value of a 16 bit register
    //
    // The following macro pattern breaks down like this:
    // operate_8bit_register!(b => self.add)
    ($getter:ident => $self:ident.$fn:ident => $setter:ident) => {{
        let register = $self.registers.$getter();
        let new_value = $self.$fn(register);
        $self.registers.$setter(new_value);
    }};
}

/// # Compund Macros
///
/// These macros make it easy to operate on 8 or 16 bit registers but store them into other registers.
///
/// Perform an arithmetic instruction on a register without storing the result.
macro_rules! perform_arithmetic {
    // For a given register, operate on an 8 bit register to set flags, but do not return the result
    ($register:ident, $self:ident.$fn:ident) => {
        {
            match $register {
                ArithmeticTarget::A => operate_8bit_register!(a => $self.$fn),
                ArithmeticTarget::B => operate_8bit_register!(b => $self.$fn),
                ArithmeticTarget::C => operate_8bit_register!(c => $self.$fn),
                ArithmeticTarget::D => operate_8bit_register!(d => $self.$fn),
                ArithmeticTarget::E => operate_8bit_register!(e => $self.$fn),
                ArithmeticTarget::H => operate_8bit_register!(h => $self.$fn),
                ArithmeticTarget::L => operate_8bit_register!(l => $self.$fn),
                ArithmeticTarget::HLI => todo!(),
            }
        }
    };

    // For a given register, perform 8 bit arithmetic from a given register and store it into the `a` register.
    ($register:ident, $self:ident.$fn:ident => a) => {
        {
            match $register {
                ArithmeticTarget::A => operate_8bit_register!(a => $self.$fn => a),
                ArithmeticTarget::B => operate_8bit_register!(b => $self.$fn => a),
                ArithmeticTarget::C => operate_8bit_register!(c => $self.$fn => a),
                ArithmeticTarget::D => operate_8bit_register!(d => $self.$fn => a),
                ArithmeticTarget::E => operate_8bit_register!(e => $self.$fn => a),
                ArithmeticTarget::H => operate_8bit_register!(h => $self.$fn => a),
                ArithmeticTarget::L => operate_8bit_register!(l => $self.$fn => a),
                ArithmeticTarget::HLI => todo!(),
            }
        }
    };
}

macro_rules! prefix_instruction {
    ($target:ident, $self:ident.$fn:ident @ $bit_position:ident) => {
        {
            match $target {
                PrefixTarget::A => operate_8bit_register!(a => $self.$fn @ $bit_position),
                PrefixTarget::B => operate_8bit_register!(b => $self.$fn @ $bit_position),
                PrefixTarget::C => operate_8bit_register!(c => $self.$fn @ $bit_position),
                PrefixTarget::D => operate_8bit_register!(d => $self.$fn @ $bit_position),
                PrefixTarget::E => operate_8bit_register!(e => $self.$fn @ $bit_position),
                PrefixTarget::H => operate_8bit_register!(h => $self.$fn @ $bit_position),
                PrefixTarget::L => operate_8bit_register!(l => $self.$fn @ $bit_position),
                PrefixTarget::HLI => todo!(),
            }
        }
    };

    ($target:ident, ($self:ident.$fn:ident @ $bit_position:ident) => register) => {
        {
            match $target {
                PrefixTarget::A => operate_8bit_register!(a => ($self.$fn @ $bit_position) => a),
                PrefixTarget::B => operate_8bit_register!(b => ($self.$fn @ $bit_position) => b),
                PrefixTarget::C => operate_8bit_register!(c => ($self.$fn @ $bit_position) => c),
                PrefixTarget::D => operate_8bit_register!(d => ($self.$fn @ $bit_position) => d),
                PrefixTarget::E => operate_8bit_register!(e => ($self.$fn @ $bit_position) => e),
                PrefixTarget::H => operate_8bit_register!(h => ($self.$fn @ $bit_position) => h),
                PrefixTarget::L => operate_8bit_register!(l => ($self.$fn @ $bit_position) => l),
                PrefixTarget::HLI => todo!(),
            }
        }
    };

    ($target:ident, $self:ident.$fn:ident => register) => {
        {
            match $target {
                PrefixTarget::A => operate_8bit_register!(a => $self.$fn => a),
                PrefixTarget::B => operate_8bit_register!(b => $self.$fn => b),
                PrefixTarget::C => operate_8bit_register!(c => $self.$fn => c),
                PrefixTarget::D => operate_8bit_register!(d => $self.$fn => d),
                PrefixTarget::E => operate_8bit_register!(e => $self.$fn => e),
                PrefixTarget::H => operate_8bit_register!(h => $self.$fn => h),
                PrefixTarget::L => operate_8bit_register!(l => $self.$fn => l),
                PrefixTarget::HLI => todo!(),
            }
        }
    };
}

pub struct CPU {
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,
    pub memory: Memory,
}

// CPU instruction functions
impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>, rom: Vec<u8>) -> Self {
        CPU {
            pc: 0,
            sp: 0,
            registers: Registers::new(),
            memory: Memory::new(boot_rom, rom),
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(register) => perform_arithmetic!(register, self.add => a),
            Instruction::ADC(register) => perform_arithmetic!(register, self.adc => a),
            Instruction::AND(register) => perform_arithmetic!(register, self.and => a),
            Instruction::BIT(target, bit_position) => {
                prefix_instruction!(target, self.bit @ bit_position);
            }
            Instruction::CP(register) => perform_arithmetic!(register, self.compare),
            Instruction::DEC(register) => match register {
                IncDecTarget::A => operate_8bit_register!(a => self.dec => a),
                IncDecTarget::B => operate_8bit_register!(b => self.dec => b),
                IncDecTarget::C => operate_8bit_register!(c => self.dec => c),
                IncDecTarget::D => operate_8bit_register!(d => self.dec => d),
                IncDecTarget::E => operate_8bit_register!(e => self.dec => e),
                IncDecTarget::H => operate_8bit_register!(h => self.dec => h),
                IncDecTarget::L => operate_8bit_register!(l => self.dec => l),
                IncDecTarget::BC => operate_16bit_register!(get_bc => self.dec_16bit => set_bc),
                IncDecTarget::DE => operate_16bit_register!(get_de => self.dec_16bit => set_de),
                IncDecTarget::HL => operate_16bit_register!(get_hl => self.dec_16bit => set_hl),
                IncDecTarget::HLI => todo!(),
                IncDecTarget::SP => todo!(),
            },
            Instruction::INC(register) => match register {
                IncDecTarget::A => operate_8bit_register!(a => self.inc => a),
                IncDecTarget::B => operate_8bit_register!(b => self.inc => b),
                IncDecTarget::C => operate_8bit_register!(c => self.inc => c),
                IncDecTarget::D => operate_8bit_register!(d => self.inc => d),
                IncDecTarget::E => operate_8bit_register!(e => self.inc => e),
                IncDecTarget::H => operate_8bit_register!(h => self.inc => h),
                IncDecTarget::L => operate_8bit_register!(l => self.inc => l),
                IncDecTarget::BC => operate_16bit_register!(get_bc => self.inc_16bit => set_bc),
                IncDecTarget::DE => operate_16bit_register!(get_de => self.inc_16bit => set_de),
                IncDecTarget::HL => operate_16bit_register!(get_hl => self.inc_16bit => set_hl),
                IncDecTarget::HLI => todo!(),
                IncDecTarget::SP => todo!(),
            },
            Instruction::OR(register) => perform_arithmetic!(register, self.or => a),
            Instruction::SBC(register) => perform_arithmetic!(register, self.sbc => a),
            Instruction::SET(target, bit_position) => {
                prefix_instruction!(target, (self.set @ bit_position) => register);
            }

            Instruction::SUB(register) => perform_arithmetic!(register, self.sub => a),
            Instruction::XOR(register) => perform_arithmetic!(register, self.xor => a),
            Instruction::ADDHL(register) => {
                let value: u16 = match register {
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => todo!(),
                };

                let result = self.add_hl(value);
                self.registers.set_hl(result);
            }
            Instruction::CCF => self.ccf(),
            Instruction::CPL => operate_8bit_register!(a => self.complement => a),
            Instruction::SCF => self.scf(),
            Instruction::SWAP(target) => prefix_instruction!(target, self.swap => register),
            Instruction::RL(target) => prefix_instruction!(target, self.rl => register),
            Instruction::RLA => operate_8bit_register!(a => self.rla => a),
            Instruction::RLC(target) => prefix_instruction!(target, self.rlc => register),
            Instruction::RLCA => operate_8bit_register!(a => self.rlca => a),
            Instruction::RR(target) => prefix_instruction!(target, self.rr => register),
            Instruction::RRA => operate_8bit_register!(a => self.rra => a),
            Instruction::RRC(target) => prefix_instruction!(target, self.rrc => register),
            Instruction::RRCA => operate_8bit_register!(a => self.rrca => a),
            Instruction::LD(load_type) => match load_type {
                LoadType::BYTE(target, source) => {
                    let source_value = match source {
                        LoadByteTarget::A => self.registers.a,
                        LoadByteTarget::B => self.registers.b,
                        LoadByteTarget::C => self.registers.c,
                        LoadByteTarget::D => self.registers.d,
                        LoadByteTarget::E => self.registers.e,
                        LoadByteTarget::H => self.registers.h,
                        LoadByteTarget::L => self.registers.l,
                        LoadByteTarget::HLI => todo!(),
                    };

                    match target {
                        LoadByteSource::A => self.registers.a = source_value,
                        LoadByteSource::B => self.registers.b = source_value,
                        LoadByteSource::C => self.registers.c = source_value,
                        LoadByteSource::D => self.registers.d = source_value,
                        LoadByteSource::E => self.registers.e = source_value,
                        LoadByteSource::H => self.registers.h = source_value,
                        LoadByteSource::L => self.registers.l = source_value,
                        LoadByteSource::HLI => todo!(),
                    }
                } // LoadType::WORD(target) => match target {
                  //     LoadWordTarget::BC => {
                  //         operate_16bit_register!(get_bc => self.load_16bit => set_bc)
                  //     }
                  //     LoadWordTarget::DE => {
                  //         operate_16bit_register!(get_de => self.load_16bit => set_de)
                  //     }
                  //     LoadWordTarget::HL => {
                  //         operate_16bit_register!(get_hl => self.load_16bit => set_hl)
                  //     }
                  // },
            },
        }
    }

    // 8 bit instructions

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        new_value
    }

    fn adc(&mut self, value: u8) -> u8 {
        let carry = u8::from(self.registers.f.carry);
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let (new_value_with_carry, did_carry_overflow) = new_value.overflowing_add(carry);

        self.registers.f.zero = new_value_with_carry == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + carry) > 0xF;
        self.registers.f.carry = did_overflow || did_carry_overflow;

        new_value_with_carry
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;

        new_value
    }

    fn compare(&mut self, value: u8) {
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // Half Carry:
        // If the operand value is greater than the register value, it will cause an underflow
        // i.e. carry the 4th bit down to the 3rd bit.
        // register_a = 0xF
        // value = 0x10
        // 0xF - 0x10 = 0xFF (carry bit set)
        // 0x10 - 0xF = 1 (carry bit not set)
        self.registers.f.half_carry = (value & 0xF) > (self.registers.a & 0xF);
        self.registers.f.carry = did_underflow;
    }

    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // Half Carry:
        // If the lower bit of the nibble are 0 (i.e. 0x10, 0x20, etc.), then subtracting 1 will
        // underflow i.e. carry the 4th bit down to the 3rd bit.
        // 0x10 - 1 = 0xF (carry bit set)
        // 0xA - 1 = 0xB (carry bit not set)
        self.registers.f.half_carry = value & 0xF == 0;

        new_value
    }

    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Carry:
        // If the lower bits are 1 (i.e. 0xAF, 0xF, etc.), then adding 1 will overflow
        // i.e. carry the 3rd bit up to the 4th bit.
        // 0xF + 1 = 0x10 (carry bit set)
        // 0xBE + 1 = 0xBF (carry bit not set)
        self.registers.f.half_carry = value & 0xF == 0xF;

        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let carry = u8::from(self.registers.f.carry);
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        let (new_value_with_carry, did_carry_underflow) = new_value.overflowing_sub(carry);

        self.registers.f.zero = new_value_with_carry == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) + carry > (self.registers.a & 0xF);
        self.registers.f.carry = did_underflow || did_carry_underflow;

        new_value_with_carry
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) > (self.registers.a & 0xF);
        self.registers.f.carry = did_underflow;

        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    // 16 bit instructions

    fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (new_value, did_overflow) = hl.overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;

        // This mask is used to detect if the number will flip the 11th bit over to the 12th.
        // The value and the hl register value are masked by 11 bits and if 1 more was added, it would flip to the 12th bit.
        // Example:
        // (0x400 & 0x7FF) + (0x400 & 0x7FF) == 0x800 (0b1000_0000_0000) (half carry set)
        // (0x3FF & 0x7FF) + (0x400 & 0x7FF) == 0x7FF (0b0111_1111_1111) (half carry not set)
        let mask = 0b111_1111_1111; // 0x7FF
        self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

        new_value
    }

    fn inc_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn dec_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    // Bit instructions

    fn bit(&mut self, value: u8, bit_position: BitPosition) {
        self.registers.f.zero = ((value >> u8::from(bit_position)) & 0b1) == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    fn set(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        value | 1 << u8::from(bit_position)
    }

    fn swap(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xF) << 4) | ((value & 0xF0) >> 4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    // Bit shift instructions

    fn rl(&mut self, value: u8) -> u8 {
        let carry = u8::from(self.registers.f.carry);
        let highest_bit = (value & 0xFF) >> 7;
        let new_value = ((value & 0xFF) << 1) | carry;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = highest_bit == 1;

        new_value
    }

    fn rla(&mut self, value: u8) -> u8 {
        let new_value = self.rl(value);
        self.registers.f.zero = false;

        new_value
    }

    fn rlc(&mut self, value: u8) -> u8 {
        let highest_bit = (value & 0xFF) >> 7;
        let new_value = ((value & 0xFF) << 1) | highest_bit;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = highest_bit == 1;

        new_value
    }

    fn rlca(&mut self, value: u8) -> u8 {
        let new_value = self.rlc(value);
        self.registers.f.zero = false;

        new_value
    }

    fn rr(&mut self, value: u8) -> u8 {
        let carry = u8::from(self.registers.f.carry);
        let lowest_bit = value & 0x1;
        let new_value = (lowest_bit << 7) | (value >> 1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lowest_bit == 1;

        new_value
    }

    fn rra(&mut self, value: u8) -> u8 {
        let new_value = self.rr(value);
        self.registers.f.zero = false;

        new_value
    }

    fn rrc(&mut self, value: u8) -> u8 {
        let lowest_bit = value & 0x1;
        let new_value = value >> 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = lowest_bit == 1;

        new_value
    }

    fn rrca(&mut self, value: u8) -> u8 {
        let new_value = self.rrc(value);
        self.registers.f.zero = false;

        new_value
    }

    // Misc instructions

    fn ccf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    fn complement(&mut self, value: u8) -> u8 {
        let new_value = !value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;

        new_value
    }

    fn scf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }
}

#[path = "./tests/cpu_tests.rs"]
mod tests;
