use std::error::Error;
use std::ops::Range;
use itertools::Itertools;
use nara_assembler_infrastructure::arch_def::{Architecture, Instruction, OperandKind, Symbol};
use nara_assembler_infrastructure::assembler::{AssemblerPass, AssemblerPasses};
use nara_assembler_infrastructure::assembler::passes::parse::PlausibleOperator;

#[derive(Clone)]
enum SisaI {}

impl Architecture for SisaI {
    type Instruction = SisaIInstruction;
    type OperandKind = SisaIOperandKind;
    type Symbol = SisaISymbol;
}

#[derive(Clone, Copy)]
enum SisaIInstruction {
    LogicArithmetic(u8),
    Comparison(u8),
    Addi,
    Ld,
    St,
    Movi,
    Movhi,
    Bz,
    Bnz,
    In,
    Out
}

impl SisaIInstruction {
    const ALL: &'static[Self] = &[
        SisaIInstruction::LogicArithmetic(0),
        SisaIInstruction::LogicArithmetic(0),
        SisaIInstruction::LogicArithmetic(1),
        SisaIInstruction::LogicArithmetic(2),
        SisaIInstruction::LogicArithmetic(3),
        SisaIInstruction::LogicArithmetic(4),
        SisaIInstruction::LogicArithmetic(5),
        SisaIInstruction::LogicArithmetic(6),
        SisaIInstruction::LogicArithmetic(7),
        SisaIInstruction::Comparison(0),
        SisaIInstruction::Comparison(1),
        SisaIInstruction::Comparison(3),
        SisaIInstruction::Comparison(4),
        SisaIInstruction::Comparison(5),
        SisaIInstruction::Addi,
        SisaIInstruction::Ld,
        SisaIInstruction::St,
        SisaIInstruction::Movi,
        SisaIInstruction::Movhi,
        SisaIInstruction::Bz,
        SisaIInstruction::Bnz,
        SisaIInstruction::In,
        SisaIInstruction::Out,
    ];
}

impl Instruction<SisaI> for SisaIInstruction {
    fn name(&self) -> &str {
        match self {
            SisaIInstruction::LogicArithmetic(0) => "and",
            SisaIInstruction::LogicArithmetic(1) => "or",
            SisaIInstruction::LogicArithmetic(2) => "xor",
            SisaIInstruction::LogicArithmetic(3) => "not",
            SisaIInstruction::LogicArithmetic(4) => "add",
            SisaIInstruction::LogicArithmetic(5) => "sub",
            SisaIInstruction::LogicArithmetic(6) => "sha",
            SisaIInstruction::LogicArithmetic(7) => "shl",
            SisaIInstruction::Comparison(0) => "cmplt",
            SisaIInstruction::Comparison(1) => "cmple",
            SisaIInstruction::Comparison(3) => "cmpeq",
            SisaIInstruction::Comparison(4) => "cmpltu",
            SisaIInstruction::Comparison(5) => "cmpleu",
            SisaIInstruction::Addi => "addi",
            SisaIInstruction::Ld => "ld",
            SisaIInstruction::St => "st",
            SisaIInstruction::Movi => "movi",
            SisaIInstruction::Movhi => "movhi",
            SisaIInstruction::Bz => "bz",
            SisaIInstruction::Bnz => "bnz",
            SisaIInstruction::In => "in",
            SisaIInstruction::Out => "out",
            _ => unreachable!(),
        }
    }

    fn operands(&self) -> impl IntoIterator<Item=SisaIOperandKind> {
        match self {
            SisaIInstruction::LogicArithmetic(3) => vec![SisaIOperandKind::Reg, SisaIOperandKind::Reg],
            SisaIInstruction::LogicArithmetic(_) => vec![SisaIOperandKind::Reg, SisaIOperandKind::Reg, SisaIOperandKind::Reg],
            SisaIInstruction::Comparison(_) => vec![SisaIOperandKind::Reg, SisaIOperandKind::Reg, SisaIOperandKind::Reg],
            SisaIInstruction::Addi => vec![SisaIOperandKind::Reg, SisaIOperandKind::Reg, SisaIOperandKind::Imm6s],
            SisaIInstruction::Ld => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm6s, SisaIOperandKind::Reg],
            SisaIInstruction::St => vec![SisaIOperandKind::Imm6s, SisaIOperandKind::Reg, SisaIOperandKind::Reg],
            SisaIInstruction::Movi => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm8s],
            SisaIInstruction::Movhi => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm8s],
            SisaIInstruction::Bz => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm8s],
            SisaIInstruction::Bnz => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm8s],
            SisaIInstruction::In => vec![SisaIOperandKind::Reg, SisaIOperandKind::Imm8u],
            SisaIInstruction::Out => vec![SisaIOperandKind::Imm8u, SisaIOperandKind::Reg],
        }
    }

    fn emit(&self, operands: impl IntoIterator<Item=SisaIOperand>) -> impl IntoIterator<Item=u8> {
        let mut instruction: u16 = 0;
        
        instruction |= match self {
            SisaIInstruction::LogicArithmetic(_) => 0,
            SisaIInstruction::Comparison(_) => 1,
            SisaIInstruction::Addi => 2,
            SisaIInstruction::Ld => 3,
            SisaIInstruction::St => 4,
            SisaIInstruction::Movi | SisaIInstruction::Movhi => 5,
            SisaIInstruction::Bz | SisaIInstruction::Bnz => 6,
            SisaIInstruction::In | SisaIInstruction::Out => 7,
        } << 12;
        
        match self {
            SisaIInstruction::LogicArithmetic(3) => {
                let Some((SisaIOperand::Reg(rd), SisaIOperand::Reg(ra))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (3 & 0b111) << 3;
                instruction |= (ra as u16 & 0b111) << 6;
                instruction |= (rd as u16 & 0b111) << 9;
            }
            SisaIInstruction::LogicArithmetic(f) | SisaIInstruction::Comparison(f) => {
                let Some((SisaIOperand::Reg(rd), SisaIOperand::Reg(ra), SisaIOperand::Reg(rb))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (rb as u16 & 0b111) << 0;
                instruction |= (*f as u16 & 0b111) << 3;
                instruction |= (ra as u16 & 0b111) << 6;
                instruction |= (rd as u16 & 0b111) << 9;
            }
            SisaIInstruction::Addi => {
                let Some((SisaIOperand::Reg(rd), SisaIOperand::Reg(ra), SisaIOperand::Imm6(imm))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (imm as u16 & 0b111111) << 0;
                instruction |= (ra as u16 & 0b111) << 6;
                instruction |= (rd as u16 & 0b111) << 9;
            }
            SisaIInstruction::Ld => {
                let Some((SisaIOperand::Reg(rd), SisaIOperand::Imm6(off), SisaIOperand::Reg(ra))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (off as u16 & 0b111111) << 0;
                instruction |= (ra as u16 & 0b111) << 6;
                instruction |= (rd as u16 & 0b111) << 9;
            }
            SisaIInstruction::St => {
                let Some((SisaIOperand::Imm6(off), SisaIOperand::Reg(ra), SisaIOperand::Reg(rb))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (off as u16 & 0b111111) << 0;
                instruction |= (ra as u16 & 0b111) << 6;
                instruction |= (rb as u16 & 0b111) << 9;
            }
            SisaIInstruction::Movi | SisaIInstruction::Movhi | SisaIInstruction::Bz | SisaIInstruction::Bnz | SisaIInstruction::In => {
                let Some((SisaIOperand::Reg(r), SisaIOperand::Imm8(imm))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (imm as u16 & 0b11111111) << 0;
                instruction |= (r as u16 & 0b111) << 9;
            }
            SisaIInstruction::Out => {
                let Some((SisaIOperand::Imm8(imm), SisaIOperand::Reg(r))) = operands.into_iter().collect_tuple() else { unreachable!() };
                instruction |= (imm as u16 & 0b11111111) << 0;
                instruction |= (r as u16 & 0b111) << 9;
            }
        }
        
        if matches!(self, SisaIInstruction::Movhi | SisaIInstruction::Bnz | SisaIInstruction::Out) {
            instruction |= 1 << 8;
        }
        
        instruction.to_le_bytes()
    }

    fn enumerate() -> impl IntoIterator<Item=&'static Self> {
        Self::ALL
    }
}

enum SisaIOperandKind {
    Reg,
    Imm6s,
    Imm8s,
    Imm8u,
}

impl OperandKind<SisaI> for SisaIOperandKind {
    type Operand = SisaIOperand;

    fn parse(&self, plausible_operator: PlausibleOperator<SisaI>) -> Result<Self::Operand, Box<dyn Error>> {
        const I6_RANGE: Range<i8> = -2i8.pow(5)..2i8.pow(5);

        match (self, plausible_operator) {
            (Self::Reg, PlausibleOperator::Symbol(SisaISymbol::Reg(reg))) => Ok(SisaIOperand::Reg(reg)),
            (Self::Imm6s, PlausibleOperator::Value(value)) => {
                let value: i8 = value.try_into()?;

                if I6_RANGE.contains(&value) {
                    Ok(SisaIOperand::Imm6(value as u8))
                } else {
                    Err("Invalid immediate value".into())
                }
            }
            (Self::Imm8s, PlausibleOperator::Value(value)) => Ok(SisaIOperand::Imm8(i8::try_from(value)? as u8)),
            (Self::Imm8u, PlausibleOperator::Value(value)) => Ok(SisaIOperand::Imm8(value.try_into()?)),
            _ => Err("Invalid operand".into()),
        }
    }
}

#[derive(Clone)]
enum SisaIOperand {
    Reg(u8),
    Imm6(u8),
    Imm8(u8),
}

#[derive(Clone)]
enum SisaISymbol {
    Reg(u8),
}

impl Symbol<SisaI> for SisaISymbol {
    fn parse(symbol: &str) -> Result<Self, Box<dyn Error>> {
        if symbol.starts_with('r') {
            if let Ok(register) = symbol[1..].parse() {
                return if register < 8 {
                    Ok(Self::Reg(register))
                } else { Err("Invalid register".into()) }
            }
        }

        Err("Invalid symbol".into())
    }
}

fn main() {
    let input = r"
        ld r1, 0, r3
        bz r1, 2
        add r2, r0, r1
        and r1, r2, r3
    ";
    
    let mut assembler_passes = AssemblerPasses::<SisaI>::default();
    
    let bytes = assembler_passes.apply_all(input.chars());
    let bytes = bytes.into_iter().collect_vec();
    
    println!("{:02x?}", bytes);
}