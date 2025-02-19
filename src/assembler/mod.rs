use crate::arch_def::Architecture;
use crate::assembler::passes::retokenize::RetokenizePass;
use passes::tokenize::TokenizePass;

pub mod passes;

pub trait AssemblerPass {
    type Input;
    type Output;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item = Self::Output>;

    fn finish(&mut self) -> impl IntoIterator<Item = Self::Output> {
        vec![]
    }

    fn apply_all(
        &mut self,
        items: impl IntoIterator<Item = Self::Input>,
    ) -> impl IntoIterator<Item = Self::Output> {
        let mut transformed = vec![];

        for item in items {
            transformed.extend(self.apply(item));
        }

        transformed.extend(self.finish());
        transformed
    }
}

pub struct AssemblerPasses<A: Architecture> {
    tokenize: TokenizePass,
    retokenize: RetokenizePass<A>,
}

impl<A: Architecture> Default for AssemblerPasses<A> {
    fn default() -> Self {
        Self {
            tokenize: TokenizePass::default(),
            retokenize: RetokenizePass::default(),
        }
    }
}

impl<A: Architecture> AssemblerPass for AssemblerPasses<A> {
    type Input = <TokenizePass as AssemblerPass>::Input;
    type Output = <RetokenizePass<A> as AssemblerPass>::Output;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item = Self::Output> {
        let tokens = self.tokenize.apply(item);
        let tokens = self.retokenize.apply_all(tokens);
        tokens
    }
}
