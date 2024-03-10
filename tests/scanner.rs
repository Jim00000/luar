use luar::scanner::Scanner;
use luar::token::Token;
use luar::token::TokenType;
use std::env;

fn tokenize(code: &str) -> Vec<Token> {
    let mut scanner = Scanner::read_source(code);
    let mut tokens = vec![];
    loop {
        match scanner.next_token() {
            Some(token) => tokens.push(token.clone()),
            None => break,
        }
    }
    tokens
}

fn tokenize_from_script(script_path: &str) -> Vec<Token> {
    let lua_src = format!(
        "{}/tests/scripts/{}",
        env::current_dir().unwrap().to_str().unwrap(),
        script_path
    );
    let mut scanner = Scanner::read_script(&lua_src);
    let mut tokens = vec![];
    loop {
        match scanner.next_token() {
            Some(token) => tokens.push(token.clone()),
            None => break,
        }
    }
    tokens
}

#[cfg(test)]
mod unittest_scanner {
    use super::*;

    #[test]
    fn test_operators() {
        let tokens = tokenize(
            "+ - * / % ^ # & ~ | << >> // == ~= <= >= < > = ( ) { } [ ] :: ; : , . .. ...",
        );
        assert_eq!(tokens.get(0).unwrap().word(), "+");
        assert_eq!(tokens.get(1).unwrap().word(), "-");
        assert_eq!(tokens.get(2).unwrap().word(), "*");
        assert_eq!(tokens.get(3).unwrap().word(), "/");
        assert_eq!(tokens.get(4).unwrap().word(), "%");
        assert_eq!(tokens.get(5).unwrap().word(), "^");
        assert_eq!(tokens.get(6).unwrap().word(), "#");
        assert_eq!(tokens.get(7).unwrap().word(), "&");
        assert_eq!(tokens.get(8).unwrap().word(), "~");
        assert_eq!(tokens.get(9).unwrap().word(), "|");
        assert_eq!(tokens.get(10).unwrap().word(), "<<");
        assert_eq!(tokens.get(11).unwrap().word(), ">>");
        assert_eq!(tokens.get(12).unwrap().word(), "//");
        assert_eq!(tokens.get(13).unwrap().word(), "==");
        assert_eq!(tokens.get(14).unwrap().word(), "~=");
        assert_eq!(tokens.get(15).unwrap().word(), "<=");
        assert_eq!(tokens.get(16).unwrap().word(), ">=");
        assert_eq!(tokens.get(17).unwrap().word(), "<");
        assert_eq!(tokens.get(18).unwrap().word(), ">");
        assert_eq!(tokens.get(19).unwrap().word(), "=");
        assert_eq!(tokens.get(20).unwrap().word(), "(");
        assert_eq!(tokens.get(21).unwrap().word(), ")");
        assert_eq!(tokens.get(22).unwrap().word(), "{");
        assert_eq!(tokens.get(23).unwrap().word(), "}");
        assert_eq!(tokens.get(24).unwrap().word(), "[");
        assert_eq!(tokens.get(25).unwrap().word(), "]");
        assert_eq!(tokens.get(26).unwrap().word(), "::");
        assert_eq!(tokens.get(27).unwrap().word(), ";");
        assert_eq!(tokens.get(28).unwrap().word(), ":");
        assert_eq!(tokens.get(29).unwrap().word(), ",");
        assert_eq!(tokens.get(30).unwrap().word(), ".");
        assert_eq!(tokens.get(31).unwrap().word(), "..");
        assert_eq!(tokens.get(32).unwrap().word(), "...");
    }

    #[test]
    fn test_short_comment() {
        let tokens = tokenize(
            r##"
            -- short comment ??? >< 
            -- abcdefghijklmnopqrstuvwxyz
            "##,
        );
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_block_comment() {
        let tokens = tokenize(
            r##"
            --[[ 123 ]]
            --[=[ 123 ]=]
            --[==[ 123 ]==]
            --[[abc
            -- |-|-|-|-|-|
            ]=]
            --efg--[=[]=] 
            ]]--[===[--123--]===]
            "##,
        );
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_string() {
        let tokens = tokenize(
            r##"
            ""
            "double quote"   
            'single quote'
            'Hello World"'
            '"Hello World'
            "Hello World'"
            "'Hello World"
            "Hello ' World"
            "'Hello'World'"
            "##,
        );
        assert_eq!(tokens.get(0).unwrap().word(), "");
        assert_eq!(tokens.get(1).unwrap().word(), "double quote");
        assert_eq!(tokens.get(2).unwrap().word(), "single quote");
        assert_eq!(tokens.get(3).unwrap().word(), "Hello World\"");
        assert_eq!(tokens.get(4).unwrap().word(), "\"Hello World");
        assert_eq!(tokens.get(5).unwrap().word(), "Hello World'");
        assert_eq!(tokens.get(6).unwrap().word(), "'Hello World");
        assert_eq!(tokens.get(7).unwrap().word(), "Hello ' World");
        assert_eq!(tokens.get(8).unwrap().word(), "'Hello'World'");
        // Check token type as STRING
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::STRING);
        }
    }

    #[test]
    fn test_long_string() {
        let tokens =
            tokenize("[[this is long-string]] [=[eq1]=] [==[ 'nothing' ]==] [===[\"'\"']===]");
        assert_eq!(tokens.get(0).unwrap().word(), "this is long-string");
        assert_eq!(tokens.get(1).unwrap().word(), "eq1");
        assert_eq!(tokens.get(2).unwrap().word(), " 'nothing' ");
        assert_eq!(tokens.get(3).unwrap().word(), r##""'"'"##);
        // Check token type as STRING
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::STRING);
        }
    }

    #[test]
    fn test_decimal_constant() {
        let tokens = tokenize(
            r##"
            3
            345
            3.0
            3.1416
            314.16e-2
            0.31416
            0.31416E1
            34e1
            "##,
        );
        assert_eq!(tokens.get(0).unwrap().word(), "3");
        assert_eq!(
            tokens.get(0).unwrap().number().unwrap_or(f32::INFINITY),
            3.0
        );
        assert_eq!(tokens.get(1).unwrap().word(), "345");
        assert_eq!(
            tokens.get(1).unwrap().number().unwrap_or(f32::INFINITY),
            345.0
        );
        assert_eq!(tokens.get(2).unwrap().word(), "3.0");
        assert_eq!(
            tokens.get(2).unwrap().number().unwrap_or(f32::INFINITY),
            3.0
        );
        assert_eq!(tokens.get(3).unwrap().word(), "3.1416");
        assert_eq!(
            tokens.get(3).unwrap().number().unwrap_or(f32::INFINITY),
            3.1416
        );
        assert_eq!(tokens.get(4).unwrap().word(), "314.16e-2");
        assert_eq!(
            tokens.get(4).unwrap().number().unwrap_or(f32::INFINITY),
            314.16e-2
        );
        assert_eq!(tokens.get(5).unwrap().word(), "0.31416");
        assert_eq!(
            tokens.get(5).unwrap().number().unwrap_or(f32::INFINITY),
            0.31416
        );
        assert_eq!(tokens.get(6).unwrap().word(), "0.31416E1");
        assert_eq!(
            tokens.get(6).unwrap().number().unwrap_or(f32::INFINITY),
            0.31416E1
        );
        assert_eq!(tokens.get(7).unwrap().word(), "34e1");
        assert_eq!(
            tokens.get(7).unwrap().number().unwrap_or(f32::INFINITY),
            34e1
        );
        // Check token type as NUMBER
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::NUMBER);
        }
    }

    #[test]
    fn test_hexadecimal_constant() {
        let tokens = tokenize(
            r##"
            0xff
            0xBEBADA
            0XDEADBEEF
            "##,
        );
        assert_eq!(tokens.get(0).unwrap().word(), "0xff");
        assert_eq!(
            tokens.get(0).unwrap().number().unwrap_or(f32::INFINITY),
            0xff as f32
        );
        assert_eq!(tokens.get(1).unwrap().word(), "0xBEBADA");
        assert_eq!(
            tokens.get(1).unwrap().number().unwrap_or(f32::INFINITY),
            0xBEBADA as f32
        );
        assert_eq!(tokens.get(2).unwrap().word(), "0XDEADBEEF");
        assert_eq!(
            tokens.get(2).unwrap().number().unwrap_or(f32::INFINITY),
            (0xDEADBEEF as i64) as f32
        );
        // Check token type as NUMBER
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::NUMBER);
        }
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize(
            r##"
            and
            break
            do
            else
            elseif
            end
            false
            for
            function
            goto
            if
            in
            local
            nil
            not
            or
            repeat
            return
            then
            true
            until
            while
            "##,
        );
        assert_eq!(tokens.get(0).unwrap().word(), "and");
        assert_eq!(tokens.get(1).unwrap().word(), "break");
        assert_eq!(tokens.get(2).unwrap().word(), "do");
        assert_eq!(tokens.get(3).unwrap().word(), "else");
        assert_eq!(tokens.get(4).unwrap().word(), "elseif");
        assert_eq!(tokens.get(5).unwrap().word(), "end");
        assert_eq!(tokens.get(6).unwrap().word(), "false");
        assert_eq!(tokens.get(7).unwrap().word(), "for");
        assert_eq!(tokens.get(8).unwrap().word(), "function");
        assert_eq!(tokens.get(9).unwrap().word(), "goto");
        assert_eq!(tokens.get(10).unwrap().word(), "if");
        assert_eq!(tokens.get(11).unwrap().word(), "in");
        assert_eq!(tokens.get(12).unwrap().word(), "local");
        assert_eq!(tokens.get(13).unwrap().word(), "nil");
        assert_eq!(tokens.get(14).unwrap().word(), "not");
        assert_eq!(tokens.get(15).unwrap().word(), "or");
        assert_eq!(tokens.get(16).unwrap().word(), "repeat");
        assert_eq!(tokens.get(17).unwrap().word(), "return");
        assert_eq!(tokens.get(18).unwrap().word(), "then");
        assert_eq!(tokens.get(19).unwrap().word(), "true");
        assert_eq!(tokens.get(20).unwrap().word(), "until");
        assert_eq!(tokens.get(21).unwrap().word(), "while");
        // Check token type as KEYWORD
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::KEYWORD);
        }
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize(
            r##"
            variable
            var123__
            _0123
            _0_
            _
            __
            IF
            If
            "##,
        );
        assert_eq!(tokens.get(0).unwrap().word(), "variable");
        assert_eq!(tokens.get(1).unwrap().word(), "var123__");
        assert_eq!(tokens.get(2).unwrap().word(), "_0123");
        assert_eq!(tokens.get(3).unwrap().word(), "_0_");
        assert_eq!(tokens.get(4).unwrap().word(), "_");
        assert_eq!(tokens.get(5).unwrap().word(), "__");
        assert_eq!(tokens.get(6).unwrap().word(), "IF");
        assert_eq!(tokens.get(7).unwrap().word(), "If");
        // Check token type as IDENTIFIER
        for token in tokens.iter() {
            assert_eq!(token.token_type(), &TokenType::IDENTIFIER);
        }
    }

    #[test]
    fn test_scan_factorial_script() {
        let tokens = tokenize_from_script("factorial.lua");
        assert_eq!(tokens.get(0).unwrap().word(), "function");
        assert_eq!(tokens.get(1).unwrap().word(), "factorial");
        assert_eq!(tokens.get(2).unwrap().word(), "(");
        assert_eq!(tokens.get(3).unwrap().word(), "n");
        assert_eq!(tokens.get(4).unwrap().word(), ")");
        assert_eq!(tokens.get(5).unwrap().word(), "if");
        assert_eq!(tokens.get(6).unwrap().word(), "n");
        assert_eq!(tokens.get(7).unwrap().word(), "==");
        assert_eq!(tokens.get(8).unwrap().word(), "0");
        assert_eq!(tokens.get(9).unwrap().word(), "then");
        assert_eq!(tokens.get(10).unwrap().word(), "return");
        assert_eq!(tokens.get(11).unwrap().word(), "1");
        assert_eq!(tokens.get(12).unwrap().word(), "else");
        assert_eq!(tokens.get(13).unwrap().word(), "return");
        assert_eq!(tokens.get(14).unwrap().word(), "n");
        assert_eq!(tokens.get(15).unwrap().word(), "*");
        assert_eq!(tokens.get(16).unwrap().word(), "factorial");
        assert_eq!(tokens.get(17).unwrap().word(), "(");
        assert_eq!(tokens.get(18).unwrap().word(), "n");
        assert_eq!(tokens.get(19).unwrap().word(), "-");
        assert_eq!(tokens.get(20).unwrap().word(), "1");
        assert_eq!(tokens.get(21).unwrap().word(), ")");
        assert_eq!(tokens.get(22).unwrap().word(), "end");
        assert_eq!(tokens.get(23).unwrap().word(), "end");
        assert_eq!(tokens.get(24).unwrap().word(), "local");
        assert_eq!(tokens.get(25).unwrap().word(), "inputNumber");
        assert_eq!(tokens.get(26).unwrap().word(), "=");
        assert_eq!(tokens.get(27).unwrap().word(), "5");
        assert_eq!(tokens.get(28).unwrap().word(), "local");
        assert_eq!(tokens.get(29).unwrap().word(), "result");
        assert_eq!(tokens.get(30).unwrap().word(), "=");
        assert_eq!(tokens.get(31).unwrap().word(), "factorial");
        assert_eq!(tokens.get(32).unwrap().word(), "(");
        assert_eq!(tokens.get(33).unwrap().word(), "inputNumber");
        assert_eq!(tokens.get(34).unwrap().word(), ")");
        assert_eq!(tokens.get(35).unwrap().word(), "print");
        assert_eq!(tokens.get(36).unwrap().word(), "(");
        assert_eq!(tokens.get(37).unwrap().word(), "Factorial of ");
        assert_eq!(tokens.get(38).unwrap().word(), "..");
        assert_eq!(tokens.get(39).unwrap().word(), "inputNumber");
        assert_eq!(tokens.get(40).unwrap().word(), "..");
        assert_eq!(tokens.get(41).unwrap().word(), " is ");
        assert_eq!(tokens.get(42).unwrap().word(), "..");
        assert_eq!(tokens.get(43).unwrap().word(), "result");
        assert_eq!(tokens.get(44).unwrap().word(), ")");
        assert!(tokens.get(45).is_none());
    }
}
