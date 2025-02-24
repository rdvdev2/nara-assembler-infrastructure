use crate::arch_def::{Architecture, Instruction, OperandKind};
use crate::assembler::passes::parse::{ASTNode, PlausibleOperator};
use crate::assembler::AssemblerPass;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct ParseOperandsPass<A: Architecture> {
    phantom_architecture: PhantomData<A>,
}

impl<A: Architecture> Default for ParseOperandsPass<A> {
    fn default() -> Self {
        Self {
            phantom_architecture: PhantomData,
        }
    }
}

impl<A: Architecture> AssemblerPass for ParseOperandsPass<A> {
    type Input = ASTNode<A>;
    type Output = ASTNodeOperandsParsed<A>;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item=Self::Output> {
        use std::iter::once;

        match item {
            ASTNode::Instruction(inst, ops) => once(ASTNodeOperandsParsed::Instruction(inst, parse_operands(inst, ops.as_ref()))),
        }
    }
}

fn parse_operands<A: Architecture>(instruction: A::Instruction, operands: &[PlausibleOperator<A>]) -> Rc<[<A::OperandKind as OperandKind<A>>::Operand]> {
    instruction.operands().into_iter().zip(operands.iter()).map(|(kind, op)| kind.parse(op.clone()).unwrap()).collect()
}

pub enum ASTNodeOperandsParsed<A: Architecture> {
    Instruction(A::Instruction, Rc<[<A::OperandKind as OperandKind<A>>::Operand]>)
}

impl<A: Architecture> Debug for ASTNodeOperandsParsed<A> where A::Instruction: Debug, <A::OperandKind as OperandKind<A>>::Operand: Debug, A::Symbol: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNodeOperandsParsed::Instruction(inst, ops) => write!(f, "Instruction({inst:?}, {ops:?})")
        }
    }
}