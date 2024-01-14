pub const BANK_0_START: usize = 0x0000;
pub const BANK_0_END: usize = 0x3FFF;
pub const BANK_0_SIZE: usize = BANK_0_END - BANK_0_START + 1;

pub const BANK_N_START: usize = 0x4000;
pub const BANK_N_END: usize = 0x7FFF;
pub const BANK_N_SIZE: usize = BANK_N_END - BANK_N_START + 1;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

pub const EXTERNAL_RAM_START: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_START + 1;

pub const WRAM_1_START: usize = 0xC000;
pub const WRAM_1_END: usize = 0xCFFF;
pub const WRAM_1_SIZE: usize = WRAM_1_END - WRAM_1_START + 1;

pub const WRAM_2_START: usize = 0xD000;
pub const WRAM_2_END: usize = 0xDFFF;
pub const WRAM_2_SIZE: usize = WRAM_2_END - WRAM_2_START + 1;

pub const ECHO_RAM_START: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = ECHO_RAM_END - ECHO_RAM_START + 1;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub const UNUSABLE_START: usize = 0xFEA0;
pub const UNUSABLE_END: usize = 0xFEFF;
pub const UNUSABLE_SIZE: usize = UNUSABLE_END - UNUSABLE_START + 1;

pub const IO_REGISTERS_START: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = IO_REGISTERS_END - IO_REGISTERS_START + 1;

pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFEE;
pub const HRAM_SIZE: usize = HRAM_END - HRAM_START + 1;

pub const INTERRUPT_ENABLE: usize = 0xFFFF;

pub struct InterruptFlags {
    pub vblank: bool,
    pub stat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
}

impl InterruptFlags {
    pub fn new() -> Self {
        InterruptFlags {
            vblank: false,
            stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }
}

pub struct Memory {
    pub bus: [u8; 0xFFFF],
    pub interrupt_flags: InterruptFlags,
}

impl Memory {
    pub fn new(boot_rom: Option<Vec<u8>>, rom: Vec<u8>) -> Self {
        let rom_size = rom.as_slice().len();
        let mut bus: [u8; 0xFFFF] = [0xFF; 0xFFFF];

        bus[0x0000..(BANK_0_SIZE + BANK_N_SIZE)].copy_from_slice(
            rom.as_slice().try_into().expect(
                format!(
                    "Rom size {} bigger than allowed rom size of {}",
                    rom_size,
                    (BANK_0_SIZE + BANK_N_SIZE)
                )
                .as_str(),
            ),
        );

        Memory {
            bus,
            interrupt_flags: InterruptFlags::new(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.bus[address as usize]
    }

    pub fn read_byte_range(&self, range: std::ops::Range<u16>) -> Vec<u8> {
        let mut result = Vec::with_capacity((range.end - range.start) as usize);

        for value in range.start..=range.end {
            result.push(self.read_byte(value));
        }

        result
    }

    pub fn verify_logo(&self) {
        let logo_data = self.read_byte_range(0x0104..0x0133);

        let matches = self
            .official_logo()
            .iter()
            .all(|&byte| logo_data.contains(&byte));

        if !matches {
            panic!("Rom header doesn't contain official logo!");
        }
    }

    fn official_logo(&self) -> Vec<u8> {
        vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ]
    }
}
