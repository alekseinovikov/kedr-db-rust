#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::tokenizer::new_tokenizer;
    use crate::tokenizer::tokens::TokenType;
    use crate::tokenizer::key_token::KeyWordToken;

    #[test]
    fn test_parse_create() {
        let tokenizer = new_tokenizer();
        let tokens = tokenizer.parse("  \n CrEaTE \t \t  \n \n");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].get_type(), TokenType::KeyWord);

        let key_word_token: &KeyWordToken = tokens[0].as_any().downcast_ref().expect("Must be KeyWordToken");
        assert_eq!(key_word_token.key_word_chars(), "create")
    }
}