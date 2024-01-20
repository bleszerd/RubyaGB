use std::{fs, os::unix::fs::MetadataExt};

/**
 * The file is named as "cpu" but are a Singleton,
 * should be splitted latter.
 */

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

const SANITIZE_MOST_SIGNIFICANT_BYTE: u16 = 0xFF00;
const SANITIZE_LESS_SIGNIFICANT_BYTE: u16 = 0xFF;

/** === Licensee code === */
const LICENSEE_NONE: u8 = 0x00;
const LICENSEE_NINTENDO_RD1: u8 = 0x01;
const LICENSEE_CAPCOM: u8 = 0x08;
const LICENSEE_ELETRONIC_ARTS: u8 = 0x13;
const LICENSEE_HUDSON_SOFT: u8 = 0x18;
const LICENSEE_B_AI: u8 = 0x19;
const LICENSEE_KSS: u8 = 0x20;
const LICENSEE_POW: u8 = 0x22;
const LICENSEE_PCM_COMPLETE: u8 = 0x24;
const LICENSEE_SAN_X: u8 = 0x25;
const LICENSEE_KEMCO_JAPAN: u8 = 0x28;
const LICENSEE_SETA: u8 = 0x29;
const LICENSEE_VIACOM: u8 = 0x30;
const LICENSEE_NINTENDO: u8 = 0x31;
const LICENSEE_BANDAI: u8 = 0x32;
const LICENSEE_OCEAN_ACCLAIM: u8 = 0x33;
const LICENSEE_KONAMI: u8 = 0x34;
const LICENSEE_HECTOR: u8 = 0x35;
const LICENSEE_TAITO: u8 = 0x37;
const LICENSEE_HUDSON: u8 = 0x38;
const LICENSEE_BANPRESTO: u8 = 0x39;
const LICENSEE_UBISOFT: u8 = 0x41;
const LICENSEE_ATLUS: u8 = 0x42;
const LICENSEE_MALIBU: u8 = 0x44;
const LICENSEE_ANGEL: u8 = 0x46;
const LICENSEE_BULLET_PROOF: u8 = 0x47;
const LICENSEE_IREM: u8 = 0x49;
const LICENSEE_ABSOLUTE: u8 = 0x50;
const LICENSEE_ACCLAIM: u8 = 0x51;
const LICENSEE_ACTIVISION: u8 = 0x52;
const LICENSEE_AMERICAN_SAMMY: u8 = 0x53;
const LICENSEE_KONAMI_2: u8 = 0x54;
const LICENSEE_HITECH_ENTERTAINMENT: u8 = 0x55;
const LICENSEE_LJN: u8 = 0x56;
const LICENSEE_MATCHBOX: u8 = 0x57;
const LICENSEE_MATTEL: u8 = 0x58;
const LICENSEE_MILTON_BRADLEY: u8 = 0x59;
const LICENSEE_TITUS: u8 = 0x60;
const LICENSEE_VIRGIN: u8 = 0x61;
const LICENSEE_LUCAS_ARTS: u8 = 0x64;
const LICENSEE_OCEAN: u8 = 0x67;
const LICENSEE_ELETRONIC_ARTS_2: u8 = 0x69;
const LICENSEE_INFOGRAMES: u8 = 0x70;
const LICENSEE_INTERPLAY: u8 = 0x71;
const LICENSEE_BRODERBUND: u8 = 0x72;
const LICENSEE_SCULPTURED: u8 = 0x73;
const LICENSEE_SCI: u8 = 0x75;
const LICENSEE_THQ: u8 = 0x78;
const LICENSEE_ACCOLADE: u8 = 0x79;
const LICENSEE_MISAWA: u8 = 0x80;
const LICENSEE_LOZC: u8 = 0x83;
const LICENSEE_TOKUMA_SHOTEN_INTERMEDIA: u8 = 0x86;
const LICENSEE_TSUKUDA_ORIGINAL: u8 = 0x87;
const LICENSEE_CHUNSOFT: u8 = 0x91;
const LICENSEE_VIDEO_SYSTEM: u8 = 0x92;
const LICENSEE_OCEAN_ACCLAIM_2: u8 = 0x93;
const LICENSEE_VARIE: u8 = 0x95;
const LICENSEE_YONEZAWA_SPAL: u8 = 0x96;
const LICENSEE_KANEKO: u8 = 0x97;
const LICENSEE_PACK_IN_SOFT: u8 = 0x99;
const LICENSEE_BOTTOM_UP: u8 = 0x9A;
const LICENSEE_KONAMI_YU_GI_OH: u8 = 0xA4;
/** === Licenssee code === */

/** === Cartridge type === */
const CARTRIDGE_TYPE_ROM_ONLY: u8 = 0x00;
const CARTRIDGE_TYPE_MBC1: u8 = 0x01;
const CARTRIDGE_TYPE_MBC1_RAM: u8 = 0x02;
const CARTRIDGE_TYPE_MBC1_RAM_BATTERY: u8 = 0x03;
const CARTRIDGE_TYPE_MBC2: u8 = 0x05;
const CARTRIDGE_TYPE_MBC2_BATTERY: u8 = 0x06;
const CARTRIDGE_TYPE_ROM_RAM1: u8 = 0x08;
const CARTRIDGE_TYPE_ROM_RAM1_BATTERY: u8 = 0x09;
const CARTRIDGE_TYPE_MMM01: u8 = 0x0B;
const CARTRIDGE_TYPE_MMM01_RAM: u8 = 0x0C;
const CARTRIDGE_TYPE_MMM01_RAM_BATTERY: u8 = 0x0D;
const CARTRIDGE_TYPE_MBC3_TIMER_BATTERY: u8 = 0x0F;
const CARTRIDGE_TYPE_MBC3_TIMER_RAM_BATTERY2: u8 = 0x10;
const CARTRIDGE_TYPE_MBC3: u8 = 0x11;
const CARTRIDGE_TYPE_MBC3_RAM2: u8 = 0x12;
const CARTRIDGE_TYPE_MBC3_RAM_BATTERY2: u8 = 0x13;
const CARTRIDGE_TYPE_MBC5: u8 = 0x19;
const CARTRIDGE_TYPE_MBC5_RAM: u8 = 0x1A;
const CARTRIDGE_TYPE_MBC5_RAM_BATTERY: u8 = 0x1B;
const CARTRIDGE_TYPE_MBC5_RUMBLE: u8 = 0x1C;
const CARTRIDGE_TYPE_MBC5_RUMBLE_RAM: u8 = 0x1D;
const CARTRIDGE_TYPE_MBC5_RUMBLE_RAM_BATTERY: u8 = 0x1E;
const CARTRIDGE_TYPE_MBC6: u8 = 0x20;
const CARTRIDGE_TYPE_MBC7_SENSOR_RUMBLE_RAM_BATTERY: u8 = 0x22;
const CARTRIDGE_TYPE_POCKET_CAMERA: u8 = 0xFC;
const CARTRIDGE_TYPE_BANDAI_TAMA5: u8 = 0xFD;
const CARTRIDGE_TYPE_HU_C3: u8 = 0xFE;
const CARTRIDGE_TYPE_HU_C1_RAM_BATTERY: u8 = 0xFF;
/** === Cartridge type === */

#[derive(Debug)]
enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADDC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RRLA(),
    CPL(),
    BIT(ArithmeticTarget),
    RESET(ArithmeticTarget),
    SET(ArithmeticTarget),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(PrefixTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
}

#[derive(Debug)]
enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug)]
enum IncDecTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
enum PrefixTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

#[derive(Debug)]
enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Debug)]
enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Debug)]
enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

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

pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

#[derive(Debug)]

pub struct CartridgeHeader {
    // pub entry: [u8; 0x4],
    pub logo: Vec<u8>,
    pub title: String,
    // pub new_lic_code: u16,
    pub lic_code: u8,
    // pub sbg_flag: u8,
    pub r#type: u8,
    pub cartridge_size: u8,
    // pub ram_size: u8,
    // pub dest_code: u8,
    pub version: u8,
    // pub checksum: u8,
    // pub global_checksum: u16,
}

struct CartridgeContext {
    filename: [char; 1024],
    cartridge_size: u32,
    cartridge_data: Vec<u8>,
    cartridge_header: &'static CartridgeHeader,
}

impl CartridgeHeader {
    pub fn licensee_bytes_from_cartridge_header(&self, bytes: &[u8]) -> u8 {
        let byte_value = bytes[0] ^ bytes[1];

        bytes[0] ^ bytes[1]
    }

    fn licensee_brand_from_cartridge_header(&self, bytes: &[u8]) -> String {
        let byte_value = self.licensee_bytes_from_cartridge_header(bytes);

        match byte_value {
            LICENSEE_NONE => String::from("NONE"),
            LICENSEE_NINTENDO_RD1 => String::from("NINTENDO_RD1"),
            LICENSEE_CAPCOM => String::from("CAPCOM"),
            LICENSEE_ELETRONIC_ARTS => String::from("ELETRONIC_ARTS"),
            LICENSEE_HUDSON_SOFT => String::from("HUDSON_SOFT"),
            LICENSEE_B_AI => String::from("B_AI"),
            LICENSEE_KSS => String::from("KSS"),
            LICENSEE_POW => String::from("POW"),
            LICENSEE_PCM_COMPLETE => String::from("PCM_COMPLETE"),
            LICENSEE_SAN_X => String::from("SAN_X"),
            LICENSEE_KEMCO_JAPAN => String::from("KEMCO_JAPAN"),
            LICENSEE_SETA => String::from("SETA"),
            LICENSEE_VIACOM => String::from("VIACOM"),
            LICENSEE_NINTENDO => String::from("NINTENDO"),
            LICENSEE_BANDAI => String::from("BANDAI"),
            LICENSEE_OCEAN_ACCLAIM => String::from("OCEAN_ACCLAIM"),
            LICENSEE_KONAMI => String::from("KONAMI"),
            LICENSEE_HECTOR => String::from("HECTOR"),
            LICENSEE_TAITO => String::from("TAITO"),
            LICENSEE_HUDSON => String::from("HUDSON"),
            LICENSEE_BANPRESTO => String::from("BANPRESTO"),
            LICENSEE_UBISOFT => String::from("UBISOFT"),
            LICENSEE_ATLUS => String::from("ATLUS"),
            LICENSEE_MALIBU => String::from("MALIBU"),
            LICENSEE_ANGEL => String::from("ANGEL"),
            LICENSEE_BULLET_PROOF => String::from("BULLET_PROOF"),
            LICENSEE_IREM => String::from("IREM"),
            LICENSEE_ABSOLUTE => String::from("ABSOLUTE"),
            LICENSEE_ACCLAIM => String::from("ACCLAIM"),
            LICENSEE_ACTIVISION => String::from("ACTIVISION"),
            LICENSEE_AMERICAN_SAMMY => String::from("AMERICAN_SAMMY"),
            LICENSEE_KONAMI_2 => String::from("KONAMI_2"),
            LICENSEE_HITECH_ENTERTAINMENT => String::from("HITECH_ENTERTAINMENT"),
            LICENSEE_LJN => String::from("LJN"),
            LICENSEE_MATCHBOX => String::from("MATCHBOX"),
            LICENSEE_MATTEL => String::from("MATTEL"),
            LICENSEE_MILTON_BRADLEY => String::from("MILTON_BRADLEY"),
            LICENSEE_TITUS => String::from("TITUS"),
            LICENSEE_VIRGIN => String::from("VIRGIN"),
            LICENSEE_LUCAS_ARTS => String::from("LUCAS_ARTS"),
            LICENSEE_OCEAN => String::from("OCEAN"),
            LICENSEE_ELETRONIC_ARTS_2 => String::from("ELETRONIC_ARTS_2"),
            LICENSEE_INFOGRAMES => String::from("INFOGRAMES"),
            LICENSEE_INTERPLAY => String::from("INTERPLAY"),
            LICENSEE_BRODERBUND => String::from("BRODERBUND"),
            LICENSEE_SCULPTURED => String::from("SCULPTURED"),
            LICENSEE_SCI => String::from("SCI"),
            LICENSEE_THQ => String::from("THQ"),
            LICENSEE_ACCOLADE => String::from("ACCOLADE"),
            LICENSEE_MISAWA => String::from("MISAWA"),
            LICENSEE_LOZC => String::from("LOZC"),
            LICENSEE_TOKUMA_SHOTEN_INTERMEDIA => String::from("TOKUMA_SHOTEN_INTERMEDIA"),
            LICENSEE_TSUKUDA_ORIGINAL => String::from("TSUKUDA_ORIGINAL"),
            LICENSEE_CHUNSOFT => String::from("CHUNSOFT"),
            LICENSEE_VIDEO_SYSTEM => String::from("VIDEO_SYSTEM"),
            LICENSEE_OCEAN_ACCLAIM_2 => String::from("OCEAN_ACCLAIM_2"),
            LICENSEE_VARIE => String::from("VARIE"),
            LICENSEE_YONEZAWA_SPAL => String::from("YONEZAWA_SPAL"),
            LICENSEE_KANEKO => String::from("KANEKO"),
            LICENSEE_PACK_IN_SOFT => String::from("PACK_IN_SOFT"),
            LICENSEE_BOTTOM_UP => String::from("BOTTOM_UP"),
            LICENSEE_KONAMI_YU_GI_OH => String::from("KONAMI_YU_GI_OH"),
            _ => panic!("No license code found!"),
        }
    }

    fn get_cartridge_type(type_code: u8) -> String {
        match type_code {
            CARTRIDGE_TYPE_ROM_ONLY => String::from("ROM_ONLY"),
            CARTRIDGE_TYPE_MBC1 => String::from("MBC1"),
            CARTRIDGE_TYPE_MBC1_RAM => String::from("MBC1_RAM"),
            CARTRIDGE_TYPE_MBC1_RAM_BATTERY => String::from("MBC1_RAM_BATTERY"),
            CARTRIDGE_TYPE_MBC2 => String::from("MBC2"),
            CARTRIDGE_TYPE_MBC2_BATTERY => String::from("MBC2_BATTERY"),
            CARTRIDGE_TYPE_ROM_RAM1 => String::from("ROM_RAM1"),
            CARTRIDGE_TYPE_ROM_RAM1_BATTERY => String::from("ROM_RAM1_BATTERY"),
            CARTRIDGE_TYPE_MMM01 => String::from("MMM01"),
            CARTRIDGE_TYPE_MMM01_RAM => String::from("MMM01_RAM"),
            CARTRIDGE_TYPE_MMM01_RAM_BATTERY => String::from("MMM01_RAM_BATTERY"),
            CARTRIDGE_TYPE_MBC3_TIMER_BATTERY => String::from("MBC3_TIMER_BATTERY"),
            CARTRIDGE_TYPE_MBC3_TIMER_RAM_BATTERY2 => String::from("MBC3_TIMER_RAM_BATTERY2"),
            CARTRIDGE_TYPE_MBC3 => String::from("MBC3"),
            CARTRIDGE_TYPE_MBC3_RAM2 => String::from("MBC3_RAM2"),
            CARTRIDGE_TYPE_MBC3_RAM_BATTERY2 => String::from("MBC3_RAM_BATTERY2"),
            CARTRIDGE_TYPE_MBC5 => String::from("MBC5"),
            CARTRIDGE_TYPE_MBC5_RAM => String::from("MBC5_RAM"),
            CARTRIDGE_TYPE_MBC5_RAM_BATTERY => String::from("MBC5_RAM_BATTERY"),
            CARTRIDGE_TYPE_MBC5_RUMBLE => String::from("MBC5_RUMBLE"),
            CARTRIDGE_TYPE_MBC5_RUMBLE_RAM => String::from("MBC5_RUMBLE_RAM"),
            CARTRIDGE_TYPE_MBC5_RUMBLE_RAM_BATTERY => String::from("MBC5_RUMBLE_RAM_BATTERY"),
            CARTRIDGE_TYPE_MBC6 => String::from("MBC6"),
            CARTRIDGE_TYPE_MBC7_SENSOR_RUMBLE_RAM_BATTERY => {
                String::from("MBC7_SENSOR_RUMBLE_RAM_BATTERY")
            }
            CARTRIDGE_TYPE_POCKET_CAMERA => String::from("POCKET_CAMERA"),
            CARTRIDGE_TYPE_BANDAI_TAMA5 => String::from("BANDAI_TAMA5"),
            CARTRIDGE_TYPE_HU_C3 => String::from("HU_C3"),
            CARTRIDGE_TYPE_HU_C1_RAM_BATTERY => String::from("HU_C1_RAM_BATTERY"),
            _ => panic!("No cartridge type found!"),
        }
    }

    pub fn init(rom_buffer: Vec<u8>) -> CartridgeHeader {
        let mut cartridge_header = CartridgeHeader {
            cartridge_size: 0,
            lic_code: 0,
            logo: vec![],
            title: String::new(),
            r#type: 0,
            version: 0,
        };

        cartridge_header.cartridge_size = rom_buffer[0x148];
        cartridge_header.lic_code =
            cartridge_header.licensee_bytes_from_cartridge_header(&rom_buffer[0x144..=0x145]);
        cartridge_header.logo = rom_buffer[0x104..=0x133].to_vec();
        cartridge_header.title = String::from_utf8_lossy(&rom_buffer[0x134..=0x143]).to_string();
        cartridge_header.r#type = rom_buffer[0x147];
        cartridge_header.version = rom_buffer[0x14c];

        cartridge_header
    }
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & SANITIZE_MOST_SIGNIFICANT_BYTE) >> 8) as u8;
        self.c = (value & SANITIZE_LESS_SIGNIFICANT_BYTE) as u8
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    #[rustfmt::skip]
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn read_next_byte(&self, pc: &mut u16) -> u8 {
        let byte = self.read_byte(*pc + 1);
        *pc += 1;

        byte
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        /*
        See:
        - https://gekkio.fi/files/gb-docs/gbctr.pdf
        - https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        - https://gbdev.io/pandocs/CPU_Registers_and_Flags.html

        ADD (add) - add the value stored in a specific register with the value in the A register
        ADDHL (add to HL) - just like ADD except that the target is added to the HL register
        ADC (add with carry) - just like ADD except that the value of the carry flag is also added to the number
        SUB (subtract) - subtract the value stored in a specific register with the value in the A register
        SBC (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
        AND (logical and) - do a bitwise and on the value in a specific register and the value in the A register
        OR (logical or) - do a bitwise or on the value in a specific register and the value in the A register
        XOR (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
        CP (compare) - just like SUB except the result of the subtraction is not stored back into A
        INC (increment) - increment the value in a specific register by 1
        DEC (decrement) - decrement the value in a specific register by 1
        CCF (complement carry flag) - toggle the value of the carry flag
        SCF (set carry flag) - set the carry flag to true
        RRA (rotate right A register) - bit rotate A register right through the carry flag
        RLA (rotate left A register) - bit rotate A register left through the carry flag
        RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
        RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
        CPL (complement) - toggle every bit of the A register
        BIT (bit test) - test to see if a specific bit of a specific register is set
        RESET (bit reset) - set a specific bit of a specific register to 0
        SET (bit set) - set a specific bit of a specific register to 1
        SRL (shift right logical) - bit shift a specific register right by 1
        RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
        RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
        RRC (rorate right) - bit rotate a specific register right by 1 (not through the carry flag)
        RLC (rorate left) - bit rotate a specific register left by 1 (not through the carry flag)
        SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
        SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
        SWAP (swap nibbles) - switch upper and lower nibble of a specific register
        */
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let current_value = self.registers.c;
                        let new_value = self.add(current_value);

                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::ADD(target)
                        )
                    }
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::ADDHL(target)
                        )
                    }
                }
            }
            Instruction::ADDC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::ADDC(target)
                        )
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SUB(target)
                        )
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SBC(target)
                        )
                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::AND(target)
                        )
                    }
                }
            }
            Instruction::OR(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::OR(target)
                        )
                    }
                }
            }
            Instruction::XOR(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::XOR(target)
                        )
                    }
                }
            }
            Instruction::CP(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::CP(target)
                        )
                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::INC(target)
                        )
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::DEC(target)
                        )
                    }
                }
            }
            Instruction::CCF() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::CCF()
                )
            }
            Instruction::SCF() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::SCF()
                )
            }
            Instruction::RRA() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::RRA()
                )
            }
            Instruction::RLA() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::RLA()
                )
            }
            Instruction::RRCA() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::RRCA()
                )
            }
            Instruction::RRLA() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::RRLA()
                )
            }
            Instruction::CPL() => {
                //TODO: Implement instruction
                panic!(
                    "Unregistered Instruction detected!\n{:?}",
                    Instruction::CPL()
                )
            }
            Instruction::BIT(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::BIT(target)
                        )
                    }
                }
            }
            Instruction::RESET(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::RESET(target)
                        )
                    }
                }
            }
            Instruction::SET(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SET(target)
                        )
                    }
                }
            }
            Instruction::SRL(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SRL(target)
                        )
                    }
                }
            }
            Instruction::RR(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::RR(target)
                        )
                    }
                }
            }
            Instruction::RL(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::RL(target)
                        )
                    }
                }
            }
            Instruction::RRC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::RRC(target)
                        )
                    }
                }
            }
            Instruction::RLC(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::RLC(target)
                        )
                    }
                }
            }
            Instruction::SRA(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SRA(target)
                        )
                    }
                }
            }
            Instruction::SLA(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SLA(target)
                        )
                    }
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    _ => {
                        //TODO: Implement instruction
                        //TODO: Add more targets
                        panic!(
                            "Unregistered Instruction detected!\n{:?}",
                            Instruction::SWAP(target)
                        )
                    }
                }
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };

                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => self.bus.read_next_byte(&mut self.pc),
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
                }
                _ => {
                    panic!("TODO: implement other load types")
                }
            },
            _ => {
                panic!("Unregistered Instruction detected!\n{:?}", instruction)
            }
        }
    }

    /* OPC-ADD */
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;

        /*                    |------| Lower
        Register A => 0 0 0 0 1 1 1 1
        Adding C   => 0 0 0 0 0 0 0 1
                High |------|

                            |-> Half carry overflow
        Res        => 0 0 0 1 0 0 0 0
        */
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    /* OPC-JP */
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            //INFO: Gameboy is a little-endian device!
            /*
                For hex 0x00002010:                  (most significant)             (less significant)
                Little-endian (less sig first):      |      0x10,     | 0x20, 0x00, |      0x00      |
                Big-endian (most sig first):         |      0x00,     | 0x00, 0x20, |      0x10      |
            */
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;

            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
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
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            /* RLC Opc */
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),

            /* Fallback */
            _ => panic!("Prefixed 0x{:?} not implemented!", byte),
        }
    }

    fn from_not_prefixed(byte: u8) -> Option<Instruction> {
        // TODO: Search differences between 0x34 / 0x23
        // TODO: Search differences between 0x2B / 0x35

        match byte {
            /* INC Opc */
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            // 0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            // 0x34 => Some(Instruction::INC(IncDecTarget::HL)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),

            /* DEC Operations */
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            // 0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            // 0x35 => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),

            /* ==== JUST FOR TEST - DELETE THIS ==== */
            0x65 => Some(Instruction::ADD(ArithmeticTarget::C)),

            /* Fallback */
            _ => panic!("Non-prefixed 0x{:?} not implemented!", byte),
        }
    }
}
