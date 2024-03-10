use crate::keyword::Keyword;
use crate::operator::Operator;
use crate::separator::Separator;
use crate::srcpos::SrcPos;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    KEYWORD,
    OPERATOR,
    SEPARATOR,
    NUMBER,
    IDENTIFIER,
    STRING,
}

#[derive(Debug, Clone)]
pub struct Token {
    word: String,
    number: Option<f32>,
    srcpos: SrcPos,
    token_type: Option<TokenType>,
    keyword: Option<Keyword>,
    operator: Option<Operator>,
    separator: Option<Separator>,
}

impl Token {
    pub fn build(token_str: String, srcpos: &SrcPos) -> Token {
        let mut token = Token {
            word: token_str,
            number: None,
            srcpos: srcpos.clone(),
            token_type: None,
            keyword: None,
            operator: None,
            separator: None,
        };

        token.keyword = Keyword::identify(&token.word);
        token.operator = Operator::identify(&token.word);
        token.separator = Separator::identify(&token.word);

        if !token.keyword.is_none() {
            token.token_type = Some(TokenType::KEYWORD);
        } else if !token.operator.is_none() {
            token.token_type = Some(TokenType::OPERATOR);
        } else if !token.separator.is_none() {
            token.token_type = Some(TokenType::SEPARATOR);
        }
        token
    }

    pub fn build_ident_or_keyword(ident_keyword_str: String, srcpos: &SrcPos) -> Token {
        let mut token = Token::build(ident_keyword_str, srcpos);
        // set identifier if not a keyword
        if token.token_type.is_none() {
            token.token_type = Some(TokenType::IDENTIFIER);
        }
        token
    }

    pub fn build_number(number: f32, number_string: String, srcpos: &SrcPos) -> Token {
        let mut token = Token::build(number_string, srcpos);
        token.number = Some(number);
        token.token_type = Some(TokenType::NUMBER);
        token
    }

    pub fn build_string(string: String, srcpos: &SrcPos) -> Token {
        let mut token = Token::build(string, srcpos);
        token.token_type = Some(TokenType::STRING);
        token
    }
}

impl Token {
    pub fn word(&self) -> &String {
        &self.word
    }

    pub fn number(&self) -> Option<f32> {
        self.number
    }

    pub fn keyword(&self) -> Option<&Keyword> {
        self.keyword.as_ref()
    }

    pub fn operator(&self) -> Option<&Operator> {
        self.operator.as_ref()
    }

    pub fn token_type(&self) -> &TokenType {
        match &self.token_type {
            Some(token_t) => &token_t,
            None => panic!("Unknown token type for {}", self.word)
        }
    }
}
