use std::marker::PhantomData;
use crate::arch_def::{Architecture, Instruction};
use crate::assembler::AssemblerPass;
use crate::assembler::passes::parse_operands::ASTNodeOperandsParsed;

pub struct EmitPass<A: Architecture> {
    phantom_architecture: PhantomData<A>,
}

impl<A: Architecture> Default for EmitPass<A> {
    fn default() -> Self {
        Self {
            phantom_architecture: PhantomData,
        }
    }
}

impl<A: Architecture> AssemblerPass for EmitPass<A> {
    type Input = ASTNodeOperandsParsed<A>;
    type Output = u8;
    
    fn apply(&mut self, input: Self::Input) -> impl IntoIterator<Item=Self::Output> {
        match input {
            ASTNodeOperandsParsed::Instruction(inst, ops) => {
                let mut bytes = vec![];
                
                for byte in inst.emit(ops.iter().cloned()) {
                    bytes.push(byte);
                }
                
                bytes
            }
        }
    }
}