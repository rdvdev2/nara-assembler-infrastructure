use crate::arch_def::{Architecture, Instruction, Symbol};
use crate::assembler::passes::tokenize::Token;
use crate::assembler::AssemblerPass;
use std::marker::PhantomData;

pub struct RetokenizePass<A: Architecture> {
    phantom_architecture: PhantomData<A>,
}

impl<A: Architecture> Default for RetokenizePass<A> {
    fn default() -> Self {
        Self {
            phantom_architecture: PhantomData,
        }
    }
}

impl<A: Architecture> AssemblerPass for RetokenizePass<A> {
    type Input = Token;
    type Output = ArchToken<A>;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item = Self::Output> {
        use std::iter::once;

        match item {
            Token::Symbol(symbol) => once(Self::parse_symbol(symbol)),
            Token::Value(value) => once(ArchToken::Value(value)),
            Token::Comma => once(ArchToken::Comma),
            Token::LineFeed => once(ArchToken::LineFeed),
        }
    }
}

impl<A: Architecture> RetokenizePass<A> {
    fn parse_symbol(symbol: String) -> ArchToken<A> {
        A::Instruction::enumerate()
            .into_iter()
            .find(|inst| inst.name() == symbol)
            .copied()
            .map(ArchToken::Instruction)
            .unwrap_or_else(|| {
                ArchToken::Symbol(
                    Symbol::parse(&symbol)
                        .unwrap_or_else(|_| panic!("Unparsable symbol: {}", symbol)),
                )
            })
    }
}

#[derive(Debug)]
pub enum ArchToken<A: Architecture> {
    Instruction(A::Instruction),
    Symbol(A::Symbol),
    Value(isize),
    Comma,
    LineFeed,
}
