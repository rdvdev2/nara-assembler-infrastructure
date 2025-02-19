use crate::assembler::AssemblerPass;

#[derive(Default)]
pub struct TokenizePass {
    state: TokenizerState,
}

impl AssemblerPass for TokenizePass {
    type Input = char;
    type Output = Token;

    fn apply(&mut self, item: Self::Input) -> impl IntoIterator<Item=Self::Output> {
        let (next_state, output) = match (&self.state, item) {
            // Linefeed (or semicolon)
            (TokenizerState::Initial, '\n') => (TokenizerState::Initial, vec![Token::LineFeed]),
            (TokenizerState::Initial, ';') => (TokenizerState::Initial, vec![Token::LineFeed]),
            
            // Ignore whitespace
            (TokenizerState::Initial, c) if c.is_whitespace() => (TokenizerState::Initial, vec![]),
            
            // Finish tokens on whitespace or commas
            (state, '\n') => (TokenizerState::Initial, vec![state.finish_or_error(), Token::LineFeed]),
            (state, ';') => (TokenizerState::Initial, vec![state.finish_or_error(), Token::LineFeed]),
            (state, c) if c.is_whitespace() => (TokenizerState::Initial, vec![state.finish_or_error()]),
            (state, ',') => (TokenizerState::Initial, vec![state.finish_or_error(), Token::Comma]),
            
            // Tokenize symbol
            (TokenizerState::Initial, c) if c.is_alphabetic() => (TokenizerState::InSymbol(String::from(c)), vec![]),
            (TokenizerState::InSymbol(s), c) if c.is_alphanumeric() => (TokenizerState::InSymbol(s.clone() + &String::from(c)), vec![]),
            
            // Tokenize value
            (TokenizerState::Initial, c) if c.is_ascii_digit() || c == '-' => (TokenizerState::InValue(String::from(c)), vec![]),
            (TokenizerState::InValue(s), c) if c.is_ascii_digit() => (TokenizerState::InValue(s.clone() + &String::from(c)), vec![]),
            
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

#[derive(Default, Debug)]
enum TokenizerState {
    #[default]
    Initial,
    InSymbol(String),
    InValue(String),
}

#[derive(Debug)]
pub enum Token {
    Symbol(String),
    Value(isize),
    Comma,
    LineFeed
}

impl TokenizerState {
    fn finish(&self) -> Option<Token> {
        match self {
            Self::InSymbol(symbol) => Some(Token::Symbol(symbol.clone())),
            Self::InValue(value) => Some(Token::Value(value.parse().unwrap())),
            _ => None,
        }
    }
    
    fn finish_or_error(&self) -> Token {
        self.finish().unwrap_or_else(|| panic!("Unexpected token"))
    }
}