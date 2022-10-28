use crate::tokenizer::token::Token;

mod tests;
mod key_word;
mod control_symbol;
pub mod token;

const DELIMITERS: [char; 3] = [' ', '\t', '\n'];

#[derive(Clone, PartialEq, Debug)]
struct TokenizerState {
    active_tokens: Vec<Token>,
    query: String,
}

pub fn new_tokenizer(query: &str) -> Box<dyn Tokenizer> {
    Box::new(TokenizerState { active_tokens: vec![], query: query.to_owned() })
}

impl TokenizerState {
    fn refresh(&mut self) {
        self.active_tokens = Token::new_all_possible_tokens();
    }

    fn process_char(&mut self, ch: &char) -> Option<Token> {
        if DELIMITERS.contains(ch) {
            let found = self.active_tokens.iter()
                .find(|token| token.can_be_completed())
                .map(|token| {token.to_owned()});

            self.refresh();
            return found;
        }

        //remove all not accepting chars
        self.active_tokens.retain_mut(|token| { !token.try_add_char(ch) });

        None
    }
}

pub trait Tokenizer {
    fn parse(&mut self) -> Vec<Token>;
}

impl Tokenizer for TokenizerState {
    fn parse(&mut self) -> Vec<Token> {
        self.refresh();

        let mut results: Vec<Token> = vec![];
        let query = self.query.to_owned();
        query.chars().into_iter().for_each(|ch| {
            let opt_token = self.process_char(&ch);
            match opt_token {
                Some(new_token) => results.push(new_token.to_owned()),
                _ => {}
            }
        });

        results
    }
}