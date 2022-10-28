use crate::tokenizer::key_word::KeyWordValue::{Create, Table};

pub(crate) enum KeyWordValue {
    Create,
    Table,
}

#[derive(Clone, PartialEq, Debug)]
pub struct KWValue {
    pub(crate) string_representation: String,
    pub(crate) name: String,
}

pub(crate) fn all_key_words() -> Vec<KeyWordValue> {
    vec![Create,
         Table]
}

impl KeyWordValue {
    pub(crate) fn get_name(&self) -> String {
        match self {
            Create => "CREATE".to_string(),
            Table => "TABLE".to_string()
        }
    }
}
