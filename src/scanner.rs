use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::srcpos::SrcPos;
use crate::token::Token;

fn read_from_filepath(path_str: &str) -> Vec<u8> {
    let path = Path::new(path_str);

    if !path.exists() {
        panic!("File {} doesn't exist", path_str);
    }

    if !path.is_file() {
        panic!("{} isn't a file", path_str);
    }

    let file = File::open(path).expect(&format!("read {} failure", path_str));
    let rd = BufReader::new(file);
    let mut src: Vec<u8> = vec![];
    for ch in rd.bytes().map(|r| r.unwrap()) {
        src.push(ch);
    }

    src
}

pub struct Scanner {
    cur_token: Option<Token>,
    src: Vec<u8>,
    srcloc: usize,
    pos: SrcPos,
    eof: bool,
}

impl Scanner {
    fn cur_char(&mut self) -> Option<char> {
        if self.srcloc < self.src.len() {
            Some(self.src[self.srcloc] as char)
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.srcloc < self.src.len() {
            self.srcloc += 1;
            if self.srcloc == self.src.len() {
                self.eof = true;
                None
            } else {
                self.pos.update(self.src[self.srcloc - 1] as char);
                Some(self.src[self.srcloc] as char)
            }
        } else {
            self.eof = true;
            None
        }
    }

    fn skip_n_char(&mut self, n: usize) {
        for _ in 0..n {
            self.next_char();
        }
    }

    fn look_ahead_char(&self, offset: usize) -> Option<char> {
        let target_loc = self.srcloc + offset;
        if target_loc < self.src.len() {
            Some(self.src[target_loc] as char)
        } else {
            None
        }
    }

    fn look_ahead_as_string(&self, len: usize) -> Option<String> {
        if self.srcloc + len <= self.src.len() {
            let string: String = self.src[self.srcloc..self.srcloc + len]
                .into_iter()
                .map(|x| *x as char)
                .collect();
            Some(string)
        } else {
            None
        }
    }

    fn dispatch_token(&mut self, token: Option<Token>) -> &Option<Token> {
        self.cur_token = token;
        &self.cur_token
    }

    fn skip_short_comment(&mut self) {
        assert_ne!(self.cur_char().unwrap_or('\0'), '[');
        'outer: loop {
            match self.cur_char() {
                Some(ch) => {
                    match ch {
                        '\n' => {
                            self.next_char(); // skip '\n'
                            break 'outer;
                        }
                        _ => (),
                    };
                    self.next_char();
                }
                None => break,
            }
        }
    }

    fn skip_block_comment(&mut self) {
        let mut eq_count = 0;
        while self.cur_char().unwrap_or('\0') == '=' {
            eq_count += 1;
            self.next_char(); // bypass '='
        }
        assert_eq!(self.cur_char().unwrap_or('\0'), '[');
        self.next_char(); // bypass '['

        let delim = ["]", &"=".repeat(eq_count), "]"].concat();

        'match_first: loop {
            let wrap_char = self.cur_char();
            if wrap_char.is_none() {
                panic!("block comment never closes");
            } else if wrap_char.unwrap() != ']' {
                self.next_char();
                continue 'match_first;
            } else {
                match self.look_ahead_as_string(eq_count + 2) {
                    Some(window) => {
                        if window.eq(&delim) {
                            self.skip_n_char(eq_count + 2);
                            break 'match_first;
                        } else {
                            self.next_char();
                            continue 'match_first;
                        }
                    }
                    None => (),
                }
            }
        }
    }

    fn scan_string(&mut self) -> Option<String> {
        let delim_char: char = self.cur_char().unwrap();
        assert!((delim_char == '\"') || (delim_char == '\'')); // assure string pattern
        self.next_char(); // skip delimiter
        let mut string = String::from("");

        'match_delim: loop {
            match self.cur_char() {
                Some(ch) => {
                    if ch == delim_char {
                        self.next_char();
                        break 'match_delim;
                    } else {
                        match ch {
                            '\n' | '\r' | '\0' => panic!(
                                "unfinished string for `{}` at line {}:{}",
                                string,
                                self.pos.position().0,
                                self.pos.position().1
                            ),
                            _ => {
                                string.push(ch);
                                self.next_char();
                            }
                        }
                    }
                }
                None => panic!("string never cloese"),
            }
        }

        Some(string)
    }

    fn scan_long_string(&mut self) -> Option<String> {
        let mut eq_count = 0;
        while self.cur_char().unwrap_or('\0') == '=' {
            eq_count += 1;
            self.next_char(); // bypass '='
        }

        if self.cur_char().unwrap_or('\0') != '[' {
            panic!(
                "invalid long string delimiter '{}' at line {}:{}",
                self.cur_char().unwrap(),
                self.pos.position().0,
                self.pos.position().1
            );
        }
        self.next_char(); // bypass '['
        let delim = ["]", &"=".repeat(eq_count), "]"].concat();
        let mut long_string = String::from("");

        'match_first_delim: loop {
            match self.cur_char() {
                Some(ch) => {
                    if ch == ']' {
                        match self.look_ahead_as_string(eq_count + 2) {
                            Some(window) => {
                                if window.eq(&delim) {
                                    self.skip_n_char(eq_count + 2);
                                    break 'match_first_delim;
                                }
                            }
                            None => (),
                        }
                    }
                    long_string.push(ch);
                    self.next_char();
                }
                None => panic!("long string never closes"),
            }
        }

        Some(long_string)
    }

    fn scan_number(&mut self) -> Option<(f32, String)> {
        let first_digit = self.cur_char().unwrap_or('\0');
        assert!(first_digit.is_ascii_digit()); // Assure first character is ascii-digit

        if first_digit == '0' && self.look_ahead_char(1).unwrap_or('\0').to_ascii_lowercase() == 'x'
        {
            self.scan_hexadecimal_number()
        } else {
            self.scan_decimal_number()
        }
    }

    fn scan_decimal_number(&mut self) -> Option<(f32, String)> {
        assert!(self.cur_char().unwrap_or('\0').is_ascii_digit()); // Assure first character is ascii-digit
        let mut literal = String::from("");
        let mut number: f32 = 0.0;
        'exit_decimal: loop {
            let ch: char = self.cur_char().unwrap_or('\0');
            match ch {
                '0'..='9' => number = number * 10.0 + ch.to_digit(10).unwrap_or(0) as f32,
                _ => break 'exit_decimal,
            }
            literal.push(ch);
            self.next_char();
        }

        let mut fractional = false;
        let mut scientific = false;
        let ch: char = self.cur_char().unwrap_or('\0');
        match ch {
            '.' => fractional = true,
            'e' | 'E' => scientific = true,
            _ => return Some((number, literal.to_string())),
        }

        literal.push(ch); // add ('.' | 'e' | 'E') to numerical literal
        self.next_char();

        if fractional {
            let mut expm10: f32 = 0.1;
            'exit_fractional: loop {
                let ch: char = self.cur_char().unwrap_or('\0');
                match ch {
                    '0'..='9' => number += expm10 * ch.to_digit(10).unwrap_or(0) as f32,
                    'e' | 'E' => {
                        scientific = true;
                        break 'exit_fractional;
                    }
                    _ => break 'exit_fractional,
                }
                expm10 *= 0.1;
                literal.push(ch);
                self.next_char();
            }
        }

        if scientific {
            let mut exponent: f32 = 0.0;
            let mut sign: f32 = 1.0;
            'exit_scientific: loop {
                let ch = self.cur_char().unwrap_or('\0');
                match ch {
                    '0'..='9' => exponent = exponent * 10.0 + ch.to_digit(10).unwrap_or(0) as f32,
                    'e' | 'E' => (),
                    '-' => sign = -sign,
                    _ => break 'exit_scientific,
                }
                literal.push(ch);
                self.next_char();
            }
            number = number * 10.0_f32.powf(exponent * sign);
        }

        // Use official number parsing implementation of Rust to verify number
        assert!(number == literal.parse::<f32>().unwrap_or(f32::INFINITY));

        Some((number, literal.to_string()))
    }

    fn scan_hexadecimal_number(&mut self) -> Option<(f32, String)> {
        assert!(self.cur_char().unwrap_or('\0') == '0'); // Assure first character starts with '0'
        assert!(self.next_char().unwrap_or('\0').to_ascii_lowercase() == 'x'); // Assure second character starts with 'x' | 'X'

        let mut number: f32 = 0.0;
        let mut literal = String::from("0");
        literal.push(self.cur_char().unwrap_or('\0')); // push 'x' | 'X'
        self.next_char();

        'exit_hexa: loop {
            let ch: char = self.cur_char().unwrap_or('\0');
            match ch {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    number = number * 16.0 + ch.to_digit(16).unwrap_or(0) as f32
                }
                _ => break 'exit_hexa,
            }
            literal.push(ch);
            self.next_char();
        }

        Some((number, literal))
    }

    fn scan_identifier_or_keyword(&mut self) -> Option<String> {
        let mut first_ch = self.cur_char().unwrap_or('\0');
        // Assure first character starts with alphabet or underscore '_'
        assert!(first_ch.is_ascii_alphabetic() || first_ch == '_');
        let mut ident_or_keyword = String::from(first_ch);
        self.next_char();

        loop {
            let ch = self.cur_char().unwrap_or('\0');
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => ident_or_keyword.push(ch),
                _ => break,
            }
            self.next_char();
        }

        Some(ident_or_keyword)
    }
}

impl Scanner {
    pub fn read_script(path: &str) -> Scanner {
        Scanner {
            cur_token: None,
            src: read_from_filepath(path),
            srcloc: 0,
            pos: SrcPos::build(),
            eof: false,
        }
    }

    pub fn read_source(code: &str) -> Scanner {
        Scanner {
            cur_token: None,
            src: code.as_bytes().to_vec(),
            srcloc: 0,
            pos: SrcPos::build(),
            eof: false,
        }
    }
}

impl Scanner {
    pub fn is_eof(&self) -> bool {
        self.eof
    }

    pub fn current_token(&self) ->Option<&Token> {
        self.cur_token.as_ref()
    }

    pub fn next_token(&mut self) -> &Option<Token> {
        while !self.eof {
            let init_pos = self.pos.clone();
            let char_wrap = self.cur_char();
            match char_wrap {
                Some(ch) => match ch {
                    '\n' | '\r' | '\t' | ' ' => {
                        self.next_char();
                    }
                    '+' | '*' | '%' | '^' | '#' | '&' | '|' | ',' | ';' | '(' | ')' | ']' | '{'
                    | '}' => {
                        self.next_char();
                        return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                    }
                    '-' => {
                        self.next_char(); // skip '-'
                        if self.cur_char().unwrap_or('\0') != '-' {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        } else {
                            self.next_char(); // skip '-'
                            if self.cur_char().unwrap_or('\0') == '[' {
                                self.next_char(); // skip '['
                                self.skip_block_comment();
                            } else {
                                self.skip_short_comment();
                            }
                        }
                    }
                    '/' => {
                        self.next_char();
                        if self.cur_char().unwrap_or('\0') != '/' {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        } else {
                            assert_eq!(self.cur_char(), Some('/'));
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("//"), &init_pos)))
                        }
                    }
                    '=' => {
                        self.next_char();
                        if self.cur_char().unwrap_or('\0') != '=' {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        } else {
                            assert_eq!(self.cur_char(), Some('='));
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("=="), &init_pos)))
                        }
                    }
                    '<' => {
                        let next_ch = self.next_char().unwrap_or('\0');
                        if next_ch == '=' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("<="), &init_pos)))
                        } else if next_ch == '<' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("<<"), &init_pos)))
                        } else {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        }
                    }
                    '>' => {
                        let next_ch = self.next_char().unwrap_or('\0');
                        if next_ch == '=' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from(">="), &init_pos)))
                        } else if next_ch == '>' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from(">>"), &init_pos)))
                        } else {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        }
                    }
                    '~' => {
                        let next_ch = self.next_char().unwrap_or('\0');
                        if next_ch == '=' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("~="), &init_pos)))
                        } else {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        }
                    }
                    ':' => {
                        let next_ch = self.next_char().unwrap_or('\0');
                        if next_ch == ':' {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from("::"), &init_pos)))
                        } else {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        }
                    }
                    '[' => {
                        let next_ch = self.next_char().unwrap_or('\0');
                        match next_ch {
                            '[' | '=' => match self.scan_long_string() {
                                Some(string) => {
                                    return self.dispatch_token(Some(Token::build_string(string, &init_pos)))
                                }
                                None => panic!("doesn't match long string pattern"),
                            },
                            _ => {
                                return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                            }
                        };
                    }
                    '\"' | '\'' => match self.scan_string() {
                        Some(string) => {
                            return self.dispatch_token(Some(Token::build_string(string, &init_pos)))
                        }
                        None => panic!("doesn't match string pattern"),
                    },
                    '.' => {
                        self.next_char();
                        if (self.cur_char().unwrap_or('\0') == '.')
                            && (self.look_ahead_char(1).unwrap_or('\0') == '.')
                        {
                            self.skip_n_char(2);
                            return self.dispatch_token(Some(Token::build(String::from("..."), &init_pos)))
                        } else if (self.cur_char().unwrap_or('\0') == '.')
                            && (self.look_ahead_char(1).unwrap_or('\0') != '.')
                        {
                            self.next_char();
                            return self.dispatch_token(Some(Token::build(String::from(".."), &init_pos)))
                        } else {
                            return self.dispatch_token(Some(Token::build(String::from(ch), &init_pos)))
                        }
                    }
                    '0'..='9' => match self.scan_number() {
                        Some(number) => {
                            return self.dispatch_token(Some(Token::build_number(number.0, number.1, &init_pos)))
                        }
                        None => panic!("doesn't match number pattern"),
                    },
                    'a'..='z' | 'A'..='Z' | '_' => match self.scan_identifier_or_keyword() {
                        Some(ident_or_keyword) => {
                            return self.dispatch_token(Some(Token::build_ident_or_keyword(ident_or_keyword, &init_pos)))
                        }
                        None => panic!("Not a valid identifier or keyword"),
                    },
                    _ => panic!(
                        "unexpected character '{}' at line {}:{}",
                        ch,
                        init_pos.position().0,
                        init_pos.position().1
                    ),
                },
                None => return self.dispatch_token(None),
            }
        }
        self.dispatch_token(None) // if EOF
    }
}
