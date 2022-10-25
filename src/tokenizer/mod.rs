pub mod tokens;


mod processor;
mod key_token;


pub trait Tokenizer {
    fn parse(&self, chars: Vec<char>) -> Vec<Box<dyn Token>>;
}

pub trait Token: Clone {
    fn get_type(&self) -> TokenType;
    fn string_representation(&self) -> &str;
    fn try_accept_char(&mut self, ch: char) -> bool;
    fn can_be_completed_now(&self) -> bool;
    fn can_take_more(&self) -> bool;
    fn must_be_completed_now(&self) -> bool;
}

use crate::tokenizer::processor::Processor;
use crate::tokenizer::tokens::TokenType;

pub fn new_tokenizer() -> Box<dyn Tokenizer> {
    Box::new(Processor::new())
}