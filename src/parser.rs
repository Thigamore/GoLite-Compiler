use std::collections::HashMap;

use crate::lexer::Lexer;

use super::Token;
use super::ast;




// pub fn parse(lex: Lexer) -> ast::AST {
//     let mut package: String;
//     let mut main: ast::Declaration;
//     let mut func_env: HashMap<Token, ast::Literal> = HashMap::new();
//     let mut val_env: HashMap<Token, ast::Literal> = HashMap::new();
//     let mut type_env: HashMap<Token, ast::Literal> = HashMap::new();

//     // Find the package
//     lex.eat(&Token::Package);
//     if let Token::Ident(name) = lex.next_token() {
//         package = name;
//     }

//     let mut tok = lex.next_token();

//     while tok != Token::EOF {
//         match tok {
//             Token::Break => todo!(),
//             Token::Case => todo!(),
//             Token::Chan => todo!(),
//             Token::Const => todo!(),
//             Token::Continue => todo!(),
//             Token::Default => todo!(),
//             Token::Defer => todo!(),
//             Token::Else => todo!(),
//             Token::Fallthrough => todo!(),
//             Token::Func => todo!(),
//             Token::For => todo!(),
//             Token::Go => todo!(),
//             Token::Goto => todo!(),
//             Token::If => todo!(),
//             Token::Import => todo!(),
//             Token::Interface => todo!(),
//             Token::Map => todo!(),
//             Token::Package => todo!(),
//             Token::Range => todo!(),
//             Token::Return => todo!(),
//             Token::Select => todo!(),
//             Token::Struct => todo!(),
//             Token::Switch => todo!(),
//             Token::Type => todo!(),
//             Token::Var => todo!(),
//             Token::Print => todo!(),
//             Token::Println => todo!(),
//             Token::Append => todo!(),
//             Token::Len => todo!(),
//             Token::Cap => todo!(),
//             Token::Plus => todo!(),
//             Token::Minus => todo!(),
//             Token::Aster => todo!(),
//             Token::FSlash => todo!(),
//             Token::Percent => todo!(),
//             Token::Amper => todo!(),
//             Token::Or => todo!(),
//             Token::Xor => todo!(),
//             Token::LShift => todo!(),
//             Token::RShift => todo!(),
//             Token::AndNot => todo!(),
//             Token::PlusEqual => todo!(),
//             Token::MinusEqual => todo!(),
//             Token::TimesEqual => todo!(),
//             Token::DivEqual => todo!(),
//             Token::ModEqual => todo!(),
//             Token::AndEqual => todo!(),
//             Token::OrEqual => todo!(),
//             Token::XorEqual => todo!(),
//             Token::LShiftEqual => todo!(),
//             Token::RShiftEqual => todo!(),
//             Token::AndNotEqual => todo!(),
//             Token::LogAnd => todo!(),
//             Token::LogOr => todo!(),
//             Token::LArrow => todo!(),
//             Token::PlusPlus => todo!(),
//             Token::MinusMinus => todo!(),
//             Token::EqualEqual => todo!(),
//             Token::Less => todo!(),
//             Token::Greater => todo!(),
//             Token::Equal => todo!(),
//             Token::Bang => todo!(),
//             Token::BangEqual => todo!(),
//             Token::LessEqual => todo!(),
//             Token::GreaterEqual => todo!(),
//             Token::Assignment => todo!(),
//             Token::Dots => todo!(),
//             Token::LParen => todo!(),
//             Token::RParen => todo!(),
//             Token::LBrack => todo!(),
//             Token::RBrack => todo!(),
//             Token::LBrace => todo!(),
//             Token::RBrace => todo!(),
//             Token::Comma => todo!(),
//             Token::Period => todo!(),
//             Token::Semicolon => todo!(),
//             Token::Colon => todo!(),
//             Token::Int(_) => todo!(),
//             Token::Float(_) => todo!(),
//             Token::Rune(_) => todo!(),
//             Token::String(_) => todo!(),
//             Token::Bool(_) => todo!(),
//             Token::Error(_, _) => todo!(),
//             Token::Ident(_) => todo!(),
//             Token::BlankIdent => todo!(),
//             Token::EOF => todo!(),
//             Token::Empty => todo!(),
//         }
//     }

//     return None;
// }

// fn parse_decl(lex: Lexer) -> ast::Declaration {

// }