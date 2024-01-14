use crate::cpu::*;
#[cfg(test)]
use crate::CPU;

macro_rules! assert_flags {
    (
        $cpu:ident
        $(, zero : $z:expr)?
        $(, subtract : $s:expr)?
        $(, half_carry : $hc:expr)?
        $(, carry : $c:expr)?
    ) => {
        {
            $(assert_eq!($cpu.registers.f.zero, $z);)?
            $(assert_eq!($cpu.registers.f.subtract, $s);)?
            $(assert_eq!($cpu.registers.f.half_carry, $hc);)?
            $(assert_eq!($cpu.registers.f.carry, $c);)?
        }
    };
}

macro_rules! test_instruction {
    // Tests 8 bit registers against another register
    (
        $instruction:expr,
        $($($register:ident).* : $value:expr),*
        $(
            ; $target_register:ident
            => $expected:expr
        )?
    ) => {
        {
            let mut cpu = new_cpu();
            $(cpu.registers$(.$register)* = $value;)*
            cpu.execute_instruction($instruction);

            $(assert_eq!(cpu.registers.$target_register, $expected);)?

            cpu
        }
    };

    // Tests 16 bit registers against the `hl` register
    (
        $instruction:expr,
        $($value:expr => $($setter:ident).*),*
        $(=> $expected:expr)?
    ) => {
        {
            let mut cpu = new_cpu();
            $(cpu.registers$(.$setter($value))*;)*
            cpu.execute_instruction($instruction);

            $(assert_eq!(cpu.registers.get_hl(), $expected);)?

            cpu
        }
    };

    // Tests loading a value from one register into another
    ($instruction:expr, $reg_1:ident : $value:expr, $reg_2:ident) =>  {
        {
            let mut cpu = new_cpu();
            cpu.registers.$reg_1 = cpu.registers.$reg_2;
            assert_eq!(cpu.registers.$reg_2, cpu.registers.$reg_1);
        }
    }
}

fn new_cpu() -> CPU {
    CPU::new(None, vec![0; 0xFFFF])
}

#[test]
fn execute_add() {
    test_instruction!(Instruction::ADD(ArithmeticTarget::A), a: 0x1 ; a => 0x2);

    let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::C), a: 0xFF, c: 0x1 ; a => 0);
    assert_flags!(cpu, zero: true, half_carry: true, carry: true);
}

#[test]
fn execute_add_carry() {
    test_instruction!(Instruction::ADC(ArithmeticTarget::B), a: 0x1, b: 0x2, f.carry: true ; a => 0x4);

    let cpu =
        test_instruction!(Instruction::ADC(ArithmeticTarget::A), a: 0x80, f.carry: true ; a => 0x1);
    assert_flags!(cpu, zero: false, half_carry: false, carry: true);

    let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::B), a: 0xFC, b: 0x03, f.carry: true ; a => 0);
    assert_flags!(cpu, zero: true, half_carry: true, carry: true);

    let cpu =
        test_instruction!(Instruction::ADC(ArithmeticTarget::H), a: 0xEA, h: 0xA1 ; a => 0x8B);
    assert_flags!(cpu, half_carry: false, carry: true);
}

#[test]
fn execute_and() {
    let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::B), a: 0xB, b: 0 ; a => 0);
    assert_flags!(cpu, zero: true, half_carry: true);

    let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::A), a: 0xF ; a => 0xF);
    assert_flags!(cpu, half_carry: true);
}

#[test]
fn execute_compare() {
    let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::A), a: 0xA2);
    assert_flags!(cpu, zero: true, subtract: true);

    let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::B), a: 0xA, b: 0xA);
    assert_flags!(cpu, subtract: true);

    let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::C), a: 0xA2, c: 0xB3);
    assert_flags!(cpu, subtract: true, half_carry: true, carry: true);
}

#[test]
fn execute_dec() {
    let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a: 0 ; a => 0xFF);
    assert_flags!(cpu, subtract: true, half_carry: true);
}

#[test]
fn execute_inc() {
    let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a: 0xFF ; a => 0);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: true);
}

#[test]
fn execute_or() {
    test_instruction!(Instruction::OR(ArithmeticTarget::B), a: 0xB, b: 0xC ; a => 0xF);

    let cpu = test_instruction!(Instruction::OR(ArithmeticTarget::A), a: 0);
    assert_flags!(cpu, zero: true);
}

#[test]
fn execute_sub() {
    test_instruction!(Instruction::SUB(ArithmeticTarget::B), a: 0xA, b: 0x3 ; a => 0x7);

    let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::A), a: 0x1 ; a => 0);
    assert_flags!(cpu, zero: true, subtract: true);

    let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::C), a: 0x10, c: 0x1 ; a => 0xF);
    assert_flags!(cpu, subtract: true, half_carry: true);

    let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::D), a: 0, d: 0x1 ; a => 0xFF);
    assert_flags!(cpu, subtract: true, half_carry: true, carry: true);
}

#[test]
fn execute_xor() {
    test_instruction!(Instruction::XOR(ArithmeticTarget::B), a: 0x3, b: 0x4 ; a => 0x7);

    let cpu = test_instruction!(Instruction::XOR(ArithmeticTarget::A), a: 0xA ; a => 0);
    assert_flags!(cpu, zero: true);
}

#[test]
fn execute_add_hl() {
    test_instruction!(Instruction::ADDHL(ADDHLTarget::BC), 0x3FF => set_hl, 0x400 => set_bc => 0x7FF);

    // ADDHL doesn't affect zero flag
    let cpu = test_instruction!(Instruction::ADDHL(ADDHLTarget::DE), 0xFFFF => set_hl, 0x1 => set_de => 0);
    assert_flags!(cpu, zero: false, half_carry: true, carry: true);

    let cpu = test_instruction!(Instruction::ADDHL(ADDHLTarget::HL), 0x400 => set_hl => 0x800);
    assert_flags!(cpu, half_carry: true);
}

#[test]
fn execute_inc_16bit() {
    test_instruction!(Instruction::INC(IncDecTarget::HL), 0x3FF => set_hl => 0x400);
}

#[test]
fn execute_dec_16bit() {
    test_instruction!(Instruction::DEC(IncDecTarget::HL), 0x3FF => set_hl => 0x3FE);
}

#[test]
fn execute_ccf() {
    let mut cpu = new_cpu();
    cpu.registers.f.carry = true;
    cpu.execute_instruction(Instruction::CCF);

    assert_eq!(cpu.registers.f.carry, false);
}

#[test]
fn execute_cpl() {
    test_instruction!(Instruction::CPL, a: 0b1010_1010 ; a => 0b0101_0101);
}

#[test]
fn execute_scf() {
    let mut cpu = new_cpu();
    cpu.registers.f.carry = false;
    cpu.execute_instruction(Instruction::SCF);

    assert_eq!(cpu.registers.f.carry, true);
}

#[test]
fn execute_set() {
    test_instruction!(Instruction::SET(PrefixTarget::A, BitPosition::B7), a: 0 ; a => 0b1000_0000);
    test_instruction!(Instruction::SET(PrefixTarget::C, BitPosition::B3), c: 0b1001_0111 ; c => 0b1001_1111);
}

#[test]
fn execute_swap() {
    test_instruction!(Instruction::SWAP(PrefixTarget::A), a: 0b0000_1111 ; a => 0b1111_0000);
    test_instruction!(Instruction::SWAP(PrefixTarget::B), b: 0b1010_0110 ; b => 0b0110_1010);
    test_instruction!(Instruction::SWAP(PrefixTarget::C), c: 0b1111_1111 ; c => 0b1111_1111);
    test_instruction!(Instruction::SWAP(PrefixTarget::L), l: 0b1111_0000 ; l => 0b0000_1111);

    let cpu =
        test_instruction!(Instruction::SWAP(PrefixTarget::H), h: 0b000_00000 ; h => 0b0000_0000);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rl() {
    test_instruction!(Instruction::RL(PrefixTarget::A), a: 0b1100_0010, f.carry: false ; a => 0b1000_0100);

    let cpu = test_instruction!(Instruction::RL(PrefixTarget::B), b: 0b1100_0010, f.carry: true ; b => 0b1000_0101);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    let cpu = test_instruction!(Instruction::RL(PrefixTarget::C), c: 0b1000_0000, f.carry: false ; c => 0);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: false, carry: true);
}

#[test]
fn execute_rla() {
    // RLA does resets the zero flag to 0, even if the result is 0
    let cpu = test_instruction!(Instruction::RLA, a: 0b1000_0000, f.carry: false ; a => 0);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    let cpu = test_instruction!(Instruction::RLA, a: 0b1000_0000, f.carry: true ; a => 0b0000_0001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    // RLA only operates on the `a` register, `b` is not affected here
    let cpu = test_instruction!(Instruction::RLA, b: 0b1000_0000, f.carry: true ; b => 0b1000_0000);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rlc() {
    let cpu =
        test_instruction!(Instruction::RLC(PrefixTarget::A), a: 0b1000_0000 ; a => 0b0000_0001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    let cpu =
        test_instruction!(Instruction::RLC(PrefixTarget::B), b: 0b0110_0100 ; b => 0b1100_1000);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);

    let cpu = test_instruction!(Instruction::RLC(PrefixTarget::C), c: 0 ; c => 0);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rlca() {
    let cpu = test_instruction!(Instruction::RLCA, a: 0b1000_0000 ; a => 0b0000_0001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    // RLCA resets the zero flag to 0, even if the result is 0
    let cpu = test_instruction!(Instruction::RLCA, a: 0 ; a => 0);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);

    // RLCA only operates on the `a` register, the `h` register is unaffected
    let cpu = test_instruction!(Instruction::RLCA, h: 0b1000_0001 ; h => 0b1000_0001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rr() {
    let cpu =
        test_instruction!(Instruction::RR(PrefixTarget::A), a: 0b1001_1001 ; a => 0b1100_1100);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    let cpu =
        test_instruction!(Instruction::RR(PrefixTarget::B), b: 0b1001_1001 ; b => 0b1100_1100);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    let cpu = test_instruction!(Instruction::RR(PrefixTarget::C), c: 0 ; c => 0);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rra() {
    let cpu = test_instruction!(Instruction::RRA, a: 0b1001_1001 ; a => 0b1100_1100);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    // RRA resets the zero flag to 0, even if the result is 0
    let cpu = test_instruction!(Instruction::RRA, a: 0 ; a => 0);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);

    // RLCA only operates on the `a` register, the `h` register is unaffected
    let cpu = test_instruction!(Instruction::RRA, b: 0b1001_1001 ; b => 0b1001_1001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rrc() {
    let cpu = test_instruction!(Instruction::RRC(PrefixTarget::A), a: 0b0000_0001 ; a => 0);
    assert_flags!(cpu, zero: true, subtract: false, half_carry: false, carry: true);

    let cpu =
        test_instruction!(Instruction::RRC(PrefixTarget::B), b: 0b1000_0000 ; b => 0b0100_0000);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_rrca() {
    // RRA resets the zero flag to 0, even if the result is 0
    let cpu = test_instruction!(Instruction::RRCA, a: 0b0000_0001 ; a => 0);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: true);

    // RRCA only operates on the `a` register, the `d` register is unaffected
    let cpu = test_instruction!(Instruction::RRCA, d: 0b1001_1001 ; d => 0b1001_1001);
    assert_flags!(cpu, zero: false, subtract: false, half_carry: false, carry: false);
}

#[test]
fn execute_ld_8bit() {
    test_instruction!(Instruction::LD(LoadType::BYTE(LoadByteSource::A, LoadByteTarget::B)), a: 0xFF, b);
    test_instruction!(Instruction::LD(LoadType::BYTE(LoadByteSource::D, LoadByteTarget::L)), d: 0xCA, l);
}
