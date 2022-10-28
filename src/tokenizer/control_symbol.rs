use crate::tokenizer::control_symbol::ControlSymbolValue::{CloseBracket, Comma, OpenBracket};

pub(crate) enum ControlSymbolValue {
    OpenBracket,
    CloseBracket,
    Comma,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CSValue{
    pub(crate) value: char,
    pub(crate) collected: bool,
}

pub(crate) fn all_control_symbols() -> Vec<ControlSymbolValue> {
    vec![OpenBracket,
         CloseBracket,
         Comma]
}

impl ControlSymbolValue {
    pub(crate) fn get_value(&self) -> char {
        match self {
            OpenBracket => '(',
            CloseBracket => ')',
            Comma => ',',
        }
    }
}