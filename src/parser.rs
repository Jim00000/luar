use crate::{
    keyword::Keyword,
    operator::Operator,
    scanner::{self, Scanner},
    token::{Token, TokenType},
};

pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn get_binop_priority(&self, op: &Operator) -> u32 {
        match op {
            Operator::ADD => 10,
            Operator::MUL => 14,
            Operator::GT => 3,
            _ => 0,
        }
    }

    fn parse_sub_expression(&mut self, priority: u32) -> Option<Operator> {
        self.parse_simple_expression();

        let mut cur_priority: u32 = 0;
        let mut bin_op: Operator;

        if self.scanner.current_token().is_some() {
            match self.scanner.current_token().unwrap().token_type() {
                TokenType::OPERATOR => {
                    bin_op = self
                        .scanner
                        .current_token()
                        .unwrap()
                        .operator()
                        .unwrap()
                        .clone();
                    cur_priority = self.get_binop_priority(&bin_op);
                    println!("parse binop {}", cur_priority);
                    if cur_priority <= priority {
                        return Some(bin_op);
                    }
                }
                _ => return Some(Operator::NOT_OPERATOR),
            }
        } else {
            // Not operator case. Only contain operand
            return Some(Operator::NOT_OPERATOR);
        }

        while cur_priority != 0 && cur_priority > priority {
            self.scanner.next_token();
            match self.parse_sub_expression(cur_priority) {
                Some(op) => {
                    println!("binop: {:?}", bin_op);
                    bin_op = op;
                }
                None => break,
            }
            cur_priority = self.get_binop_priority(&bin_op);
        }

        if bin_op != Operator::NOT_OPERATOR {
            println!("binop: {:?}", bin_op);
        }

        return Some(bin_op);
    }

    fn parse_simple_expression(&mut self) {
        let token = self.scanner.current_token().unwrap();
        match token.token_type() {
            TokenType::NUMBER => {
                println!("load {}", token.number().unwrap());
                self.scanner.next_token();
            }
            TokenType::STRING => {
                println!("load str {}", token.word());
            }
            TokenType::KEYWORD => match token.keyword().as_ref().unwrap() {
                Keyword::TRUE => {
                    println!("load true");
                }
                Keyword::FALSE => {
                    println!("load false");
                }
                Keyword::NIL => {
                    println!("load nil");
                }
                _ => (),
            },
            TokenType::IDENTIFIER => {
                println!("load id {}", token.word());
            }
            _ => (),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        self.scanner.next_token();
        self.parse_sub_expression(0);
    }
}

impl Parser {
    pub fn read_script(path: &str) -> Parser {
        Parser {
            scanner: Scanner::read_script(path),
        }
    }

    pub fn read_source(code: &str) -> Parser {
        Parser {
            scanner: Scanner::read_source(code),
        }
    }
}
