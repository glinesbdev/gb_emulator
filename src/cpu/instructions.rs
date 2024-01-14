use std;

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

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    HLI,
    SP,
}

pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

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

impl std::convert::From<BitPosition> for u8 {
    fn from(value: BitPosition) -> Self {
        match value {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}

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

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
}

pub enum LoadType {
    BYTE(LoadByteSource, LoadByteTarget),
    // WORD(LoadWordTarget),
}

pub enum RSTVector {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

pub enum Interrupts {
    VBLANK,
    STAT,
    TIMER,
    SERIAL,
    JOYPAD,
}

pub enum Instruction {
    // 8 bit instructions
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    AND(ArithmeticTarget),
    CP(ArithmeticTarget),
    DEC(IncDecTarget),
    INC(IncDecTarget),
    OR(ArithmeticTarget),
    SBC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    XOR(ArithmeticTarget),

    // 16 bit instructions
    ADDHL(ADDHLTarget),

    // Bit operations
    BIT(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SWAP(PrefixTarget),

    // Bit shift instructions
    RL(PrefixTarget),
    RLA,
    RLC(PrefixTarget),
    RLCA,
    RR(PrefixTarget),
    RRA,
    RRC(PrefixTarget),
    RRCA,

    // Load instructions
    LD(LoadType),

    // Jumps and Subroutines
    // RST(RSTVector),

    // Misc instructions
    CCF,
    CPL,
    SCF,
}
