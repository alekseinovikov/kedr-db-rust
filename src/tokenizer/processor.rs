use crate::tokenizer::{Token, Tokenizer};
use crate::tokenizer::tokens::TokenType;

pub(crate) struct Processor {
    active_tokens: Vec<Box<dyn Token>>,
}

impl Processor {
    fn available_tokens() -> Vec<Box<dyn Token>> {
        vec![TokenType::new_key_word("create")]
    }

    pub(crate) fn new() -> Processor {
        Processor {
            active_tokens: Self::available_tokens()
        }
    }
}

impl Tokenizer for Processor {
    fn parse(&self, str: &str) -> Vec<Box<dyn Token>> {
        todo!()
    }
}