#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Arithmetic  Operators
    ADD,       // addition
    SUB_MINUS, // subtraction or unary minus
    MUL,       // multiplication
    DIV,       // division
    FLRDIV,    // floor division
    MOD,       // modulo
    EXP,       // exponentiation

    // Bitwise Operators
    AND,     // bitwise AND
    OR,      // bitwise OR
    XOR_NOT, // bitwise XOR or unary bitwise NOT
    LSHIFT,  // left shift
    RSHIFT,  // right shift

    // Relational Operators
    EQ,  // equality
    NEQ, // inequality
    LT,  // less than
    GT,  // greater than
    LE,  // less or equal
    GE,  // greater or equal

    // Others
    ASSIGN, // assignment
    LENGTH, // length
    CONCAT, // concatenation
    VARARG, // variadic argument

    // Special
    NOT_OPERATOR, // Not an operator
}

impl Operator {
    pub fn identify(word: &str) -> Option<Operator> {
        match word {
            "+" => Some(Operator::ADD),
            "-" => Some(Operator::SUB_MINUS),
            "*" => Some(Operator::MUL),
            "/" => Some(Operator::DIV),
            "//" => Some(Operator::FLRDIV),
            "%" => Some(Operator::MOD),
            "^" => Some(Operator::EXP),
            "&" => Some(Operator::AND),
            "|" => Some(Operator::OR),
            "~" => Some(Operator::XOR_NOT),
            "<<" => Some(Operator::LSHIFT),
            ">>" => Some(Operator::RSHIFT),
            "==" => Some(Operator::EQ),
            "~=" => Some(Operator::NEQ),
            "<" => Some(Operator::LT),
            ">" => Some(Operator::GT),
            "<=" => Some(Operator::LE),
            ">=" => Some(Operator::GE),
            "=" => Some(Operator::ASSIGN),
            "#" => Some(Operator::LENGTH),
            ".." => Some(Operator::CONCAT),
            "..." => Some(Operator::VARARG),
            _ => None,
        }
    }
}
