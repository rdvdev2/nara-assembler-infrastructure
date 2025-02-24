use crate::arch_def::Architecture;
use crate::assembler::passes::retokenize::RetokenizePass;
use passes::tokenize::TokenizePass;
use crate::assembler::passes::emit::EmitPass;
use crate::assembler::passes::parse::ParsePass;
use crate::assembler::passes::parse_operands::ParseOperandsPass;

pub mod passes;

pub trait AssemblerPass {
    type Input;
    type Output;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item = Self::Output>;

    fn finish(&mut self) -> impl IntoIterator<Item = Self::Output> {
        vec![]
    }
    
    fn apply_all_partial(
        &mut self,
        items: impl IntoIterator<Item = Self::Input>,
    ) -> impl IntoIterator<Item = Self::Output> {
        let mut transformed = vec![];

        for item in items {
            transformed.extend(self.apply(item));
        }
        
        transformed
    }
    
    fn apply_all(
        &mut self,
        items: impl IntoIterator<Item = Self::Input>,
    ) -> impl IntoIterator<Item = Self::Output> {
        let mut transformed = Vec::from_iter(self.apply_all_partial(items));
        transformed.extend(self.finish());
        transformed
    }
}

pub struct AssemblerPasses<A: Architecture> {
    tokenize: TokenizePass,
    retokenize: RetokenizePass<A>,
    parse: ParsePass<A>,
    parse_operands: ParseOperandsPass<A>,
    emit: EmitPass<A>,
}

impl<A: Architecture> Default for AssemblerPasses<A> {
    fn default() -> Self {
        Self {
            tokenize: TokenizePass::default(),
            retokenize: RetokenizePass::default(),
            parse: ParsePass::default(),
            parse_operands: ParseOperandsPass::default(),
            emit: EmitPass::default(),
        }
    }
}

impl<A: Architecture> AssemblerPass for AssemblerPasses<A> {
    type Input = <TokenizePass as AssemblerPass>::Input;
    type Output = <EmitPass<A> as AssemblerPass>::Output;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item = Self::Output> {
        let tokens = self.tokenize.apply(item);
        let tokens = self.retokenize.apply_all_partial(tokens);
        let ast_nodes = self.parse.apply_all_partial(tokens);
        let ast_nodes = self.parse_operands.apply_all_partial(ast_nodes);
        let bytes = self.emit.apply_all_partial(ast_nodes);
        bytes
    }

    fn finish(&mut self) -> impl IntoIterator<Item=Self::Output> {
        let tokens = self.tokenize.finish();
        let tokens = self.retokenize.apply_all(tokens);
        let ast_nodes = self.parse.apply_all(tokens);
        let ast_nodes = self.parse_operands.apply_all(ast_nodes);
        let bytes = self.emit.apply_all(ast_nodes);
        bytes
    }
}
