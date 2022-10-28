use crate::tokenizer::token::Token;

mod tests;
mod key_word;
mod control_symbol;
pub mod token;

#[derive(Clone, PartialEq, Debug)]
struct TokenizerState {
    active_tokens: Vec<Token>,
}

pub fn new_tokenizer() -> Box<dyn Tokenizer> {
    Box::new(TokenizerState { active_tokens: vec![] })
}

impl TokenizerState {
    fn init(&mut self) {
        self.active_tokens = Token::new_all_possible_tokens();
    }

    fn process_char(&mut self, ch: &char) -> Option<Token> {
        None
    }
}

pub trait Tokenizer {
    fn parse(&mut self, str: &str) -> Vec<Token>;
}

impl Tokenizer for TokenizerState {
    fn parse(&mut self, str: &str) -> Vec<Token> {
        self.init();

        let mut results = vec![];
        str.chars().into_iter().for_each(|ch| {
            let opt_token = self.process_char(&ch);
            match opt_token {
                Some(new_token) => results.push(new_token),
                _ => {}
            }
        });

        results
    }
}