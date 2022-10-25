use crate::tokenizer::{Token, Tokenizer};
use crate::tokenizer::tokens::TokenType;

pub(crate) struct Processor {
    active_tokens: Vec<Box<dyn Token>>
}

impl Processor {
    const AVAILABLE_TOKENS: Vec<Box<dyn Token>> = vec![TokenType::new_key_word("create")];

    pub(crate) fn new()-> Processor {
        Processor {
            active_tokens: Self::AVAILABLE_TOKENS.clone()
        }
    }
}

impl Tokenizer for Processor {
    fn parse(&self, chars: Vec<char>) -> Vec<Box<dyn Token>> {
        todo!()
    }
}