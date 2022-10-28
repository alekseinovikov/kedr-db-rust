#[cfg(test)]
mod tests {
    use crate::tokenizer::key_word::KeyWordValue;
    use crate::tokenizer::new_tokenizer;
    use crate::tokenizer::token::Token;

    #[test]
    fn test_parse_create() {
        let mut tokenizer = new_tokenizer();
        let tokens = tokenizer.parse("  \n CrEaTE \t \t  \n \n");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new_key_word(&KeyWordValue::Create));
    }
}