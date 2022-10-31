#[cfg(test)]
mod tests {
    use crate::tokenizer::key_word::KeyWordValue;
    use crate::tokenizer::new_tokenizer;
    use crate::tokenizer::token::Token;

    #[test]
    fn test_parse_create() {
        let mut tokenizer = new_tokenizer("  \n CrEaTE \t \t  \n \n");
        let tokens = tokenizer.parse();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new_key_word(&KeyWordValue::Create, "CrEaTE"));
    }

    #[test]
    fn test_parse_table() {
        let mut tokenizer = new_tokenizer("  \t\t\t\\n TABlE \t\t\t\t \n");
        let tokens = tokenizer.parse();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new_key_word(&KeyWordValue::Table, "TABlE"));
    }
}