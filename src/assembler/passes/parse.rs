use std::fmt::{Debug, Formatter};
use crate::arch_def::Architecture;
use crate::assembler::passes::retokenize::ArchToken;
use crate::assembler::AssemblerPass;
use std::rc::Rc;

pub struct ParsePass<A: Architecture> {
    state: ParserState<A>
}

impl<A: Architecture> Default for ParsePass<A> {
    fn default() -> Self {
        Self {
            state: ParserState::default()
        }
    }
}

impl <A: Architecture> AssemblerPass for ParsePass<A> {
    type Input = ArchToken<A>;
    type Output = ASTNode<A>;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item=Self::Output> {
        let (next_state, output) = match (&self.state, item) {
            // Skip over line feeds
            (ParserState::Initial, ArchToken::LineFeed) => (ParserState::Initial, None),

            // Parse instruction
            (ParserState::Initial, ArchToken::Instruction(inst)) => (ParserState::InInstruction(InInstruction::start(inst)), None),
            (ParserState::InInstruction(inst), ArchToken::Symbol(symbol)) if inst.can_accept_operator => (ParserState::InInstruction(inst.with_operator(PlausibleOperator::Symbol(symbol))), None),
            (ParserState::InInstruction(inst), ArchToken::Value(value)) if inst.can_accept_operator => (ParserState::InInstruction(inst.with_operator(PlausibleOperator::Value(value))), None),
            (ParserState::InInstruction(inst), ArchToken::Comma) if !inst.can_accept_operator => (ParserState::InInstruction(inst.with_comma()), None),
            (state @ ParserState::InInstruction(inst), ArchToken::LineFeed) if inst.can_finish => (ParserState::Initial, Some(state.finish_or_error())),

            // Fail for anything else
            _ => panic!("Unexpected token")
        };

        self.state = next_state;
        output
    }

    fn finish(&mut self) -> impl IntoIterator<Item=Self::Output> {
        self.state.finish()
    }
}

#[derive(Default)]
enum ParserState<A: Architecture> {
    #[default]
    Initial,
    InInstruction(InInstruction<A>)
}

impl<A: Architecture> ParserState<A> {
    fn finish(&self) -> Option<ASTNode<A>> {
        match self {
            ParserState::Initial => None,
            ParserState::InInstruction(inst) => inst.finish(),
        }
    }

    fn finish_or_error(&self) -> ASTNode<A> {
        self.finish().unwrap_or_else(|| panic!("Unfinished node"))
    }
}

struct InInstruction<A: Architecture> {
    instruction: A::Instruction,
    operators: Vec<PlausibleOperator<A>>,
    can_accept_operator: bool,
    can_finish: bool,
}

impl<A: Architecture> InInstruction<A> {
    fn start(instruction: A::Instruction) -> Self {
        Self {
            instruction,
            operators: vec![],
            can_accept_operator: true,
            can_finish: true,
        }
    }

    fn with_operator(&self, operator: PlausibleOperator<A>) -> Self {
        let mut operators = self.operators.clone();
        operators.push(operator);
        Self {
            instruction: self.instruction,
            operators,
            can_accept_operator: false,
            can_finish: true,
        }
    }

    fn with_comma(&self) -> Self {
        Self {
            instruction: self.instruction,
            operators: self.operators.clone(),
            can_accept_operator: true,
            can_finish: false,
        }
    }

    fn finish(&self) -> Option<ASTNode<A>> {
        if self.can_finish {
            Some(ASTNode::Instruction(self.instruction, self.operators.clone().into()))
        } else {
            None
        }
    }
}

pub enum ASTNode<A: Architecture> {
    Instruction(A::Instruction, Rc<[PlausibleOperator<A>]>)
}

impl<A: Architecture> Debug for ASTNode<A> where A::Instruction: Debug, A::Symbol: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Instruction(inst, ops) => write!(f, "Instruction({inst:?}, {ops:?})")
        }
    }
}

#[derive(Clone)]
pub enum PlausibleOperator<A: Architecture> {
    Symbol(A::Symbol),
    Value(isize)
}

impl<A: Architecture> Debug for PlausibleOperator<A> where A::Symbol: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlausibleOperator::Symbol(symbol) => write!(f, "Symbol({symbol:?})"),
            PlausibleOperator::Value(value) => write!(f, "Value({value:?})")
        }
    }
}