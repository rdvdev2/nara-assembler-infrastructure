use nara_assembler_infrastructure::arch_def::{Architecture, Instruction, OperandKind, Symbol};
use nara_assembler_infrastructure::assembler::{AssemblerPass, AssemblerPasses};
use std::error::Error;
use std::rc::Rc;
use nara_assembler_infrastructure::assembler::passes::parse::PlausibleOperator;

#[derive(Clone, Debug)]
enum TestArch {}

#[derive(Clone, Copy, Debug)]
enum TestInstructions {
    Xor,
    Addi,
    AddiImplicit,
    Halt,
    Jump,
}

const TEST_INSTRUCTIONS: &[TestInstructions] = &[
    TestInstructions::Xor,
    TestInstructions::Addi,
    TestInstructions::AddiImplicit,
    TestInstructions::Halt,
    TestInstructions::Jump,
];

enum TestOperandKinds {
    Register,
    Immediate,
}

#[derive(Debug)]
enum TestOperands {
    Register(u8),
    Immediate(i16),
}

#[derive(Clone, Debug)]
enum TestSymbols {
    Register(u8),
}

impl Architecture for TestArch {
    type Instruction = TestInstructions;
    type OperandKind = TestOperandKinds;
    type Symbol = TestSymbols;
}

impl Instruction<TestArch> for TestInstructions {
    fn name(&self) -> &str {
        match self {
            TestInstructions::Xor => "xor",
            TestInstructions::Addi => "addi",
            TestInstructions::AddiImplicit => "addi",
            TestInstructions::Halt => "halt",
            TestInstructions::Jump => "jump",
        }
    }

    fn operands(&self) -> impl IntoIterator<Item = <TestArch as Architecture>::OperandKind> {
        match self {
            TestInstructions::Xor => vec![TestOperandKinds::Register, TestOperandKinds::Register, TestOperandKinds::Register],
            TestInstructions::Addi => vec![TestOperandKinds::Register, TestOperandKinds::Register, TestOperandKinds::Immediate],
            TestInstructions::AddiImplicit => vec![TestOperandKinds::Register, TestOperandKinds::Immediate],
            TestInstructions::Halt => vec![],
            TestInstructions::Jump => vec![TestOperandKinds::Immediate],
        }
    }

    fn emit(
        &self,
        operands: impl IntoIterator<
            Item = <TestArch as Architecture>::Symbol,
            IntoIter = impl ExactSizeIterator,
        >,
    ) -> String {
        "".into()
    }

    fn enumerate() -> impl IntoIterator<Item = &'static Self> {
        &TEST_INSTRUCTIONS[..]
    }
}

impl OperandKind<TestArch> for TestOperandKinds {
    type Operand = TestOperands;

    fn parse(&self, plausible_operator: PlausibleOperator<TestArch>) -> Result<Self::Operand, Box<dyn Error>> {
        match (self, plausible_operator) {
            (Self::Register, PlausibleOperator::Symbol(TestSymbols::Register(register))) => Ok(TestOperands::Register(register)),
            (Self::Immediate, PlausibleOperator::Value(value)) => Ok(TestOperands::Immediate(value.try_into()?)),
            _ => Err("The provided operand can't be accepted".into())
        }
    }
}

impl Symbol<TestArch> for TestSymbols {
    fn parse(symbol: &str) -> Result<Self, Box<dyn Error>> {
        if symbol.starts_with('r') {
            Ok(Self::Register(symbol[1..].parse()?))
        } else {
            Err(format!("Unparsable symbol: {}", symbol).into())
        }
    }
}

fn main() {
    let input = r"
        xor r0, r0, r0
        addi r0, r0, 1
        addi r0, 1
        halt; jump -1
    ";

    let mut assembler_passes = AssemblerPasses::<TestArch>::default();

    let ast_nodes = assembler_passes.apply_all(input.chars());

    for node in ast_nodes {
        println!("{node:?}")
    }
}
