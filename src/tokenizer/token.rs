use crate::tokenizer::control_symbol::{all_control_symbols, ControlSymbolValue, CSValue};
use crate::tokenizer::key_word::{all_key_words, KeyWordValue, KWValue};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    KeyWord {
        value: KWValue
    },

    ControlSymbol {
        value: CSValue
    },
}

impl Token {
    pub(crate) fn new_key_word(key_word: &KeyWordValue) -> Token {
        let name = key_word.get_name();
        Token::KeyWord {
            value: KWValue {
                string_representation: "".to_string(),
                name,
            }
        }
    }

    pub(crate) fn new_control_symbol(control_symbol: &ControlSymbolValue) -> Token {
        let ch_value = control_symbol.get_value();
        Token::ControlSymbol { value: CSValue { value: ch_value, collected: false } }
    }

    pub(crate) fn new_all_possible_tokens() -> Vec<Token> {
        let mut result = vec![];

        all_key_words()
            .iter()
            .for_each(|kw| { result.push(Token::new_key_word(kw)) });

        all_control_symbols()
            .iter()
            .for_each(|cs| { result.push(Token::new_control_symbol(cs)) });

        result
    }

    pub(crate) fn must_be_completed(&self) -> bool {
        match self {
            Token::KeyWord { .. } => false,
            Token::ControlSymbol { value } => value.collected
        }
    }

    pub(crate) fn can_be_completed(&self) -> bool {
        match self {
            Token::KeyWord { value } => value.name.to_ascii_lowercase() == value.string_representation.to_ascii_lowercase(),
            Token::ControlSymbol { value } => value.collected
        }
    }

    pub(crate) fn try_add_char(&mut self, ch: &char) -> bool {
        match self {
            Token::KeyWord { value } => {
                value.string_representation.push(*ch);
                if value.string_representation.to_ascii_lowercase() == value.name.to_ascii_lowercase() {
                    true
                } else {
                    false
                }
            }
            Token::ControlSymbol { value } => {
                if value.value.to_ascii_lowercase() == ch.to_ascii_lowercase() {
                    value.collected = true;
                    true
                } else {
                    false
                }
            }
        }
    }
}