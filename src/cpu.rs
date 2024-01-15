const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

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
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
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
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
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

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8
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

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
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

            _ => {
                panic!("Unregistered Instruction detected!\n{:?}", instruction)
            }
        }
    }

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
}
