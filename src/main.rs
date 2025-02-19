use nara_assembler_infrastructure::arch_def::{Architecture, Instruction, OperandKind, Symbol};
use nara_assembler_infrastructure::assembler::{AssemblerPass, AssemblerPasses};
use std::error::Error;

#[derive(Debug)]
enum TestArch {}

#[derive(Clone, Copy, Debug)]
enum TestInstructions {
    Xor,
    Addi,
    Halt,
    Jump,
}

const TEST_INSTRUCTIONS: &[TestInstructions] = &[
    TestInstructions::Xor,
    TestInstructions::Addi,
    TestInstructions::Halt,
    TestInstructions::Jump,
];

enum TestOperandKinds {}

#[derive(Debug)]
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
            TestInstructions::Halt => "halt",
            TestInstructions::Jump => "jump",
        }
    }

    fn operands(&self) -> impl IntoIterator<Item = <TestArch as Architecture>::OperandKind> {
        vec![]
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
    type Operand = ();

    fn parse(&self, s: &str) -> Result<Self::Operand, Box<dyn Error>> {
        unimplemented!()
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
        addi r0, 1
        halt; jump -1
    ";

    let mut assembler_passes = AssemblerPasses::<TestArch>::default();

    let tokens = assembler_passes.apply_all(input.chars());

    for token in tokens {
        println!("{token:?}")
    }
}
