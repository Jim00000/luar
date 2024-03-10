#[derive(Debug, Clone, Copy)]
pub enum Separator {
    COMMA,       // ,
    SEMICOLON,   // ;
    COLON,       // :
    DOUBLECOLON, // ::
    DOT,         // .
    LPAREN,      // (
    RPAREN,      // )
    LSQUAR,      // [
    RSQUAR,      // ]
    LBRACE,      // {
    RBRACE,      // }
}

impl Separator {
    pub fn identify(word: &str) -> Option<Separator> {
        match word {
            "," => Some(Separator::COMMA),
            ";" => Some(Separator::SEMICOLON),
            ":" => Some(Separator::COLON),
            "::" => Some(Separator::DOUBLECOLON),
            "." => Some(Separator::DOT),
            "(" => Some(Separator::LPAREN),
            ")" => Some(Separator::RPAREN),
            "[" => Some(Separator::LSQUAR),
            "]" => Some(Separator::RSQUAR),
            "{" => Some(Separator::LBRACE),
            "}" => Some(Separator::RBRACE),
            _ => None,
        }
    }
}
