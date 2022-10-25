use crate::tokenizer::Token;
use crate::tokenizer::tokens::TokenType;

impl TokenType {
    pub(crate) fn new_key_word(key_word_string: &str) -> Box<dyn Token> {
        let chars = key_word_string.chars().collect();
        Box::new(KeyWordToken {
            chars: vec![],
            key_word: chars,
            current_pointer: 0,
            string_representation: String::new(),
        })
    }
}

struct KeyWordToken {
    chars: Vec<char>,
    key_word: Vec<char>,
    current_pointer: usize,
    string_representation: String,
}

impl Clone for KeyWordToken {
    fn clone(&self) -> Self {
        let key_word = self.key_word.into_iter().collect();
        KeyWordToken {
            chars: vec![],
            key_word,
            current_pointer: 0,
            string_representation: String::new(),
        }
    }
}

impl Token for KeyWordToken {
    fn get_type(&self) -> TokenType {
        TokenType::KeyWord
    }

    fn string_representation(&self) -> &str {
        self.string_representation.as_str()
    }

    fn try_accept_char(&mut self, ch: char) -> bool {
        if self.key_word[self.current_pointer + 1].to_ascii_lowercase() != ch.to_ascii_lowercase() {
            return false;
        }

        self.current_pointer += 1;
        self.chars.push(ch);
        true
    }

    fn can_be_completed_now(&self) -> bool {
        self.chars.len() == self.key_word.len()
    }

    fn can_take_more(&self) -> bool {
        self.chars.len() < self.key_word.len()
    }

    fn must_be_completed_now(&self) -> bool {
        self.chars.len() == self.key_word.len()
    }
}