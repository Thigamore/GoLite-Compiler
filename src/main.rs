use std::{fs::File, io::BufReader};

use lexer::{Lexer, Token};

pub mod lexer;



fn main() {
    let f = File::open("./test.go").expect("Couldn't find file");
    let buf_reader = BufReader::new(f);
    let mut lex = Lexer::new(buf_reader);
    let mut tok = lex.next_token();
    while tok != Token::EOF {
        println!("{:?}", tok);
        tok = lex.next_token();
    }
}
