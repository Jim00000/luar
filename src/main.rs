#![allow(warnings)]

use luar;
use luar::scanner::Scanner;
use luar::parser::Parser;

fn main() {
    let mut parser = Parser::read_script("/home/jim00000/samba/bootcamp/luar/src.lua");
    parser.parse();
}
