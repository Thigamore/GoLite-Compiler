use std::collections::HashMap;

use crate::ast::ForClause;
use crate::ast::ParamList;
use crate::error;
use crate::lexer::Lexer;

use super::ast;
use super::Token;

pub fn parse(lex: Lexer) -> ast::AST {
    let mut package: String;
    let mut main: ast::Declaration;
    let mut func_env: HashMap<String, ast::FuncLiteral> = HashMap::new();
    let mut val_env: HashMap<String, ast::FuncLiteral> = HashMap::new();
    let mut type_env: HashMap<String, ast::FuncLiteral> = HashMap::new();

    // Find the package
    lex.eat(&Token::Package);
    if let Token::Ident(name) = lex.next_token() {
        package = name;
    }
    lex.eat(&Token::Semicolon);

    let mut tok = lex.next_token();

    while tok != Token::EOF {
        match tok {
            Token::Const => todo!(),
            Token::Func => {
                parse_func(&mut lex, &mut func_env);
            }
            Token::Import => {
                while !tok.same_type(&Token::Semicolon) {
                    tok = lex.next_token();
                }
            }
            Token::Interface => todo!(),
            Token::Type => todo!(),
            Token::Var => todo!(),
            _ => error::token_type_err(&tok, &Token::Func )
        }
    }

    return None;
}

fn parse_decl(lex: Lexer) -> ast::Declaration {}

fn parse_func(lex: &mut Lexer, func_env: &mut HashMap<String, ast::FuncLiteral>) {
    // Get the func name
    let mut tok = lex.next_token();
    let func_name: String;

    if let Token::Ident(name) = tok {
        func_name = name;
    } else {
        error::token_type_err(&tok, &Token::Ident("".to_string()));
    }
    lex.eat(&Token::LParen);

    let mut param_list: ast::ParamList = ParamList { params: Vec::new() };

    // Get the func parameters
    while !lex.peek().same_type(&Token::RParen) {
        tok = lex.next_token();
        let mut param_names = Vec::new();
        if let Token::Ident(name) = tok {
            param_names.push(name);
        } else {
            error::token_type_err(&tok, &Token::Ident("".to_string()));
        }

        // If there are multiple names
        while lex.peek().same_type(&Token::Comma) {
            lex.next_token();
            tok = lex.next_token();
            if let Token::Ident(name) = tok {
                param_names.push(name);
            } else {
                error::token_type_err(&tok, &Token::Ident("".to_string()));
            }
        }

        //The type of the parameters
        param_list.params.push((param_names, parse_type(lex)));
        if lex.peek().same_type(&Token::Comma) {
            lex.next_token();
        }
    }

    lex.eat(&Token::RParen);

    // Get return type if there is one
    let ret: Option<ast::Type>;

    if !(lex.peek().same_type(&Token::Ident("".to_string()))
        || lex.peek().same_type(&Token::RBrack))
    {
        ret = None;
    } else {
        ret = Some(parse_type(lex));
    }

    // Parse the block/StmtList
    let lit = ast::FuncLiteral {
        params: if param_list.params.is_empty() {
            None
        } else {
            Some(param_list)
        },
        ret,
        body: parse_stmt_list(lex),
    };
}

fn parse_type(lex: &mut Lexer) -> ast::Type {
    match lex.peek() {
        // Types
        Token::Ident(name) => {
            match name as &str {
                "int" => {
                    return ast::Type::Integer;
                }
                "float" => {
                    return ast::Type::Float;
                }
                "rune" => {
                    return ast::Type::Rune;
                }
                "bool" => {
                    return ast::Type::Bool;
                }
                "string" => {
                    return ast::Type::String;
                }
                _ => {
                    // TODO STRUCT
                    todo!();
                }
            }
        }

        // Arrays or Slices
        Token::LBrack => {
            lex.next_token();
            match lex.peek() {
                // Slice
                Token::RBrack => {
                    lex.next_token();
                    return ast::Type::Slice(Box::from(parse_type(lex)));
                }

                // Array
                Token::Int(num) => {
                    let temp = num.clone();
                    lex.next_token();
                    return ast::Type::Array(temp, Box::from(parse_type(lex)));
                }
                _ => {
                    error::token_type_err(lex.peek(), &Token::RBrack);
                    return ast::Type::Bool;
                }
            }
        }
        _ => {
            error::token_type_err(lex.peek(), &Token::Ident("".to_string()));
            return ast::Type::Bool;
        }
    }
}

fn parse_stmt_list(lex: &mut Lexer) -> ast::StmtList {
    let mut stmts = Vec::new();
    while !lex.peek().same_type(&Token::RBrace) {
        let stmt = parse_stmt(lex);
        lex.eat(&Token::Semicolon);
        stmts.push(stmt);
    }
    return ast::StmtList { stmts };
}

fn parse_stmt(lex: &mut Lexer) -> ast::Statement {
    match lex.peek() {
        Token::Print => {
            lex.eat(&Token::LParen);
            let stmt;
            if let Token::RParen = lex.peek() {
                stmt = ast::Statement::PrintStmt(ast::PrintType::Print, None);
            } else {
                stmt = ast::Statement::PrintStmt(
                    ast::PrintType::Print,
                    Some(parse_expr_list(lex, None)),
                );
            }
            lex.eat(&Token::RParen);
            lex.eat(&Token::Semicolon);
            return stmt;
        }
        Token::Println => {
            lex.eat(&Token::LParen);
            let stmt;
            if let Token::RParen = lex.peek() {
                stmt = ast::Statement::PrintStmt(ast::PrintType::Println, None);
            } else {
                stmt = ast::Statement::PrintStmt(
                    ast::PrintType::Println,
                    Some(parse_expr_list(lex, None)),
                );
            }
            lex.eat(&Token::RParen);
            lex.eat(&Token::Semicolon);
            return stmt;
        }
        Token::Return => {
            lex.next_token();
            // TODO check if it's an expr
            if let Token::Semicolon = lex.peek() {
                lex.next_token();
                return ast::Statement::ReturnStmt(None);
            } else {
                return ast::Statement::ReturnStmt(Some(parse_expr(lex)));
            }
        }
        Token::Break => {
            lex.eat(&Token::Semicolon);
            return ast::Statement::BreakStmt;
        }
        Token::Continue => {
            lex.eat(&Token::Semicolon);
            return ast::Statement::ContinueStmt;
        }
        Token::Fallthrough => {
            lex.eat(&Token::Semicolon);
            return ast::Statement::FallthroughStmt;
        }
        Token::If => {
            lex.next_token();
            let mut simple = Option::from(parse_simple_stmt(lex));
            let expr: ast::Expression;

            // Check if it is a simple stmt or just an expr
            if !lex.peek().same_type(&Token::Semicolon) {
                if let ast::SimpleStmt::ExprStmt(temp) = simple.unwrap() {
                    expr = temp;
                } else {
                    error::ast_type_err("IDK Something else", "Expression");
                }
                simple = None;
            } else {
                expr = parse_expr(lex);
            }
            lex.eat(&Token::LBrace);
            let block = parse_stmt_list(lex);
            lex.eat(&Token::RBrace);

            if lex.peek().same_type(&Token::Else) {
                lex.next_token();
                let else_stmt;
                if lex.peek().same_type(&Token::If) {
                    else_stmt = parse_stmt(lex);
                } else {
                    else_stmt = ast::Statement::StmtList(Box::new(parse_stmt_list(lex)));
                }
                return ast::Statement::IfStmt(
                    simple,
                    expr,
                    Box::new(block),
                    Some(Box::new(else_stmt)),
                );
            } else {
                return ast::Statement::IfStmt(simple, expr, Box::new(block), None);
            }
        }
        Token::Switch => {
            lex.next_token();

            // Check if there's a simple stmt or expressoin
            let mut simple;
            let expr;
            if lex.peek().same_type(&Token::LBrace) {
                simple = None;
                expr = None;
            } else {
                simple = Some(parse_simple_stmt(lex));
                if lex.peek().same_type(&Token::Semicolon) {
                    expr = Some(parse_expr(lex));
                } else {
                    if let ast::SimpleStmt::ExprStmt(expr2) = simple.unwrap() {
                        expr = Some(expr2);
                        simple = None;
                    } else {
                        error::ast_type_err("IDK Something else", "Expression");
                    }
                }
            }
            lex.eat(&Token::LBrace);
            let mut cases = Vec::new();

            // Continue while there are clauses
            while lex.peek().same_type(&Token::Case) || lex.peek().same_type(&Token::Default) {
                // Default
                if lex.peek().same_type(&Token::Default) {
                    lex.eat(&Token::Default);
                    lex.eat(&Token::Colon);
                    cases.push(ast::ExprCaseClause {
                        expr: None,
                        stmt: parse_stmt_list(lex),
                    });
                }
                // A case
                else {
                    lex.eat(&Token::Case);
                    let expr_list = parse_expr_list(lex, None);
                    lex.eat(&Token::Colon);
                    cases.push(ast::ExprCaseClause {
                        expr: Some(expr_list),
                        stmt: parse_stmt_list(lex),
                    });
                }
            }
            lex.eat(&Token::RBrace);
            return ast::Statement::SwitchStmt(simple, expr, cases);
        }
        Token::For => {
            lex.eat(&Token::For);

            // Check if it's an infinite for
            if lex.peek().same_type(&Token::LBrace) {
                lex.eat(&Token::LBrace);
                let stmt = parse_stmt_list(lex);
                lex.eat(&Token::RBrace);
                return ast::Statement::ForStmt(None, stmt);
            }

            let for_clause: ForClause;
            let simple = parse_simple_stmt(lex);

            // Determine if it's an expr or a clause
            if lex.peek().same_type(&Token::Semicolon) {
                lex.eat(&Token::Semicolon);

                // whether the Expr is empty
                if lex.peek().same_type(&Token::Semicolon) {
                    lex.eat(&Token::Semicolon);

                    // whether the second statment is empty
                    if lex.peek().same_type(&Token::LBrace) {
                        // Some, Empty, Empty
                        for_clause = ast::ForClause {
                            stmt1: Some(simple),
                            expr: None,
                            stmt2: None,
                        }
                    } else {
                        //Some, Empty, Some
                        for_clause = ast::ForClause {
                            stmt1: Some(simple),
                            expr: None,
                            stmt2: Some(parse_simple_stmt(lex))
                        }
                    }
                } else {
                    let expr = parse_expr(lex);

                    lex.eat(&Token::Semicolon);

                    // Check if third statement is empty
                    if lex.peek().same_type(&Token::RBrace) {
                        // Some, Some, Empty
                        for_clause = ast::ForClause{
                            stmt1: Some(simple),
                            expr: Some(expr),
                            stmt2: None,
                        }
                    } 
                    // Some, Some, Some
                    else {
                        for_clause = ast::ForClause {
                            stmt1: Some(simple),
                            expr: Some(expr),
                            stmt2: Some(parse_simple_stmt(lex)),
                        }
                    }
                }
            } else {
                // If there is only an expression
                if let ast::SimpleStmt::ExprStmt(expr) = simple {
                    for_clause = ast::ForClause {
                        stmt1: None,
                        expr: Some(expr),
                        stmt2: None,
                    };
                } else {
                    error::ast_type_err("IDK SOMETHING NOT RIGHT", "Expression");
                    todo!();
                }
            }
            
            // Parse the block
            lex.eat(&Token::LBrace);

            let block = parse_stmt_list(lex);
            
            lex.eat(&Token::RBrace);
            return ast::Statement::ForStmt(Some(for_clause), block);
        }
        Token::Type => {
            lex.eat(&Token::Type);

            // Single or multiple type defs
            if lex.peek().same_type(&Token::LParen) {
                // Multiple
                let mut decls = Vec::new();
                while !lex.peek().same_type(&Token::RParen) {
                    // Get the name
                    let mut ident = "".to_string();
                    if let Token::Ident(name) = lex.peek() {
                        ident = name.to_string();
                    } else {
                        error::token_type_err(lex.peek(), &Token::Ident("".to_string()));
                    }
                    lex.next_token();

                    // Get the decl and add to vec
                    decls.push((ident, parse_type(lex)));
                }
                return ast::Statement::Decl(ast::Declaration::Type(decls));

            } else if lex.peek().same_type(&Token::Ident("".to_string())) {
                // Single
                // Get name
                let mut ident = "".to_string();
                if let Token::Ident(name) = lex.next_token() {
                    ident = name;
                }

                // Get Type
                return ast::Statement::Decl(ast::Declaration::Type(vec![(ident, parse_type(lex))]));
                
            } else {
                error::token_type_err(lex.peek(), &Token::Ident("".to_string()));
                // TODO Maybe add an error thing
                todo!();
            }
        }
        Token::Var => {
            lex.eat(&Token::Var);

            // One of multiple vars
            if lex.peek().same_type(&Token::LParen) {
                // Multiple
                lex.next_token();
                
                let mut specs = Vec::new();

                // Get all the vars
                while !lex.peek().same_type(&Token::RParen) {

                    // Get the ident list
                    let mut ident_list = Vec::new();
                    while let Token::Ident(name) = lex.peek() {
                        ident_list.push(name.to_string());
                        lex.next_token();
                    }
                    
                    // Get the type
                    let var_type = parse_type(lex);

                    // Potentially get the expr list
                    if lex.peek().same_type(&Token::Equal) {
                        lex.eat(&Token::Equal);
                        specs.push(ast::VarSpec{
                            ident_list,
                            var_type,
                            expr_list: Some(parse_expr_list(lex, None)),
                        })
                    } else {
                        specs.push(ast::VarSpec{
                            ident_list,
                            var_type,
                            expr_list: None,
                        })
                    }
                    lex.eat(&Token::Semicolon);
                }

                lex.eat(&Token::RParen);
                return ast::Statement::Decl(ast::Declaration::Var(specs));

            } else {
                // One
                
                // Identifiers
                let mut ident_list = Vec::new();
                while let Token::Ident(name) = lex.peek() {
                    ident_list.push(name.to_string());
                    lex.next_token();
                }

                // Type
                let var_type = parse_type(lex);

                // Potential exprList
                if lex.peek().same_type(&Token::Equal) {
                    return ast::Statement::Decl(ast::Declaration::Var(vec![ast::VarSpec{
                        ident_list,
                        var_type,
                        expr_list: Some(parse_expr_list(lex, None)),
                    }]))
                } else {
                    return ast::Statement::Decl(ast::Declaration::Var(vec![ast::VarSpec{
                        ident_list,
                        var_type,
                        expr_list: None,
                    }]))
                }
            } 
        }
        _ => {
            return ast::Statement::SimpleStmt(parse_simple_stmt(lex));
        }
    }
}

fn parse_simple_stmt(lex: &mut Lexer) -> ast::SimpleStmt {
    match lex.peek() {
        // Empty Stmt
        Token::Semicolon => {
            lex.next_token();
            return ast::SimpleStmt::EmptyStmt;
        }
        // Short Var Decl
        Token::Ident(name) => {
            let mut ident_list = Vec::new();
            ident_list.push(name.to_string());
            lex.next_token();
            while lex.peek().same_type(&Token::Comma) {
                lex.next_token();
                if let Token::Ident(name2) = lex.peek() {
                    ident_list.push(name2.to_string());
                } else {
                    error::token_type_err(lex.peek(), &Token::Ident("".to_string()));
                }
            }
            lex.eat(&Token::Assignment);
            return ast::SimpleStmt::ShortValDecl(ident_list, parse_expr_list(lex, None));
        }

        // Something with an expr
        _ => {
            let expr = parse_expr(lex);
            match lex.peek() {
                Token::Semicolon => {
                    return ast::SimpleStmt::ExprStmt(expr);
                }
                Token::PlusPlus => {
                    return ast::SimpleStmt::IncDecStmt(expr, ast::IncDecType::Inc);
                }
                Token::MinusMinus => {
                    return ast::SimpleStmt::IncDecStmt(expr, ast::IncDecType::Dec);
                }
                Token::Comma => {
                    let expr_list = parse_expr_list(lex, Some(expr));
                    let op;
                    match lex.peek() {
                        Token::Plus => op = Some(ast::Operator::Plus),
                        Token::Minus => op = Some(ast::Operator::Minus),
                        Token::Aster => op = Some(ast::Operator::Times),
                        Token::FSlash => op = Some(ast::Operator::Divide),
                        Token::Percent => op = Some(ast::Operator::Mod),
                        Token::RShift => op = Some(ast::Operator::RShift),
                        Token::LShift => op = Some(ast::Operator::LShift),
                        Token::Xor => op = Some(ast::Operator::Xor),
                        Token::AndNot => op = Some(ast::Operator::AndNot),
                        Token::Amper => op = Some(ast::Operator::And),
                        Token::Equal => op = None,
                        _ => error::token_type_err(lex.peek(), &Token::Equal),
                    }
                    lex.eat(&Token::Equal);
                    return ast::SimpleStmt::Assignment(expr_list, op, parse_expr_list(lex, None));
                }
                _ => {
                    error::token_type_err(lex.peek(), &Token::Semicolon);

                    // TODO Maybe add like an error stmt
                    panic!();
                }
            }
        }
    }
}

fn parse_expr_list(lex: &mut Lexer, first: Option<ast::Expression>) -> ast::ExprList {
    return todo!();
}

fn parse_expr(lex: &mut Lexer) -> ast::Expression {
    return todo!();
}

fn parse_type(lex: &mut Lexer) -> ast::Type {
    return todo!();
}