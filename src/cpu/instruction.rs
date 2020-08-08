use crate::bus::Bus;
use crate::types::Result;

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Instruction {
    operation: InstructionOperation,
    mode: InstructionMode,
    len: u8,
    cycles_base: u8,
}

macro_rules! instruction {
    ($operation:ident, $mode:ident, $cycles_base:literal) => {{
        let mode = InstructionMode::$mode;

        Instruction {
            operation: InstructionOperation::$operation,
            mode,
            len: mode.len_bytes(),
            cycles_base: $cycles_base,
        }
    }};
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x69 => instruction!(Adc, Immediate, 2),
            0x65 => instruction!(Adc, ZeroPage,  3),
            0x75 => instruction!(Adc, ZeroPageX, 4),
            0x6D => instruction!(Adc, Absolute,  4),
            0x7D => instruction!(Adc, AbsoluteX, 4),
            0x79 => instruction!(Adc, AbsoluteY, 4),
            0x61 => instruction!(Adc, IndirectX, 6),
            0x71 => instruction!(Adc, IndirectY, 5),
            0x00 => instruction!(Brk, Implied,   7),
            0xA9 => instruction!(Lda, Immediate, 2),
            0xA5 => instruction!(Lda, ZeroPage,  3),
            0xB5 => instruction!(Lda, ZeroPageX, 4),
            0xAD => instruction!(Lda, Absolute,  4),
            0xBD => instruction!(Lda, AbsoluteX, 4),
            0xB9 => instruction!(Lda, AbsoluteY, 4),
            0xA1 => instruction!(Lda, IndirectX, 6),
            0xB1 => instruction!(Lda, IndirectY, 5),
            0xEA => instruction!(Nop, Implied,   2),
            _ => unimplemented!("no instruction found for opcode `${:02X}`", opcode),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionOperation {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc,
    Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp,
    Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti,
    Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionMode {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl InstructionMode {
    pub fn len_bytes(&self) -> u8 {
        match self {
            InstructionMode::Implied | InstructionMode::Accumulator => 1,
            InstructionMode::Immediate
                | InstructionMode::Relative
                | InstructionMode::ZeroPage
                | InstructionMode::ZeroPageX
                | InstructionMode::ZeroPageY
                | InstructionMode::IndirectX
                | InstructionMode::IndirectY
                => 2,
            InstructionMode::Absolute
                | InstructionMode::AbsoluteX
                | InstructionMode::AbsoluteY
                | InstructionMode::Indirect
                => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionInput {
    Implied,
    Accumulator,
    Byte(u8),
    Address(u16),
}

impl InstructionInput {
    pub fn unwrap_address(self) -> Result<u16> {
        match self {
            InstructionInput::Address(value) => Ok(value),
            _ => Err(anyhow!("`InstructionInput` is not of variant `Address`")),
        }
    }
}
