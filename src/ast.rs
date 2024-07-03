use std::collections::HashMap;

use super::Token;

pub struct AST {
    package: String,
    main: Declaration,
    func_env: HashMap<Token, Literal>,
    val_env: HashMap<Token, Literal>,
    type_env: HashMap<Token, Literal>,
}

pub enum Expression {
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    SimpleExpr(Box<SimpleExpr>),
}

pub enum SimpleExpr {
    Append(Expression),
    Length(Expression),
    Capacity(Expression),
    Operand(Operand),
    Conversion(Token, Expression),
    Selector(Box<SimpleExpr>, Token),
    Index(Box<SimpleExpr>, Expression),
    Call(Box<SimpleExpr>, Option<Token>, Option<ExprList>)
}

pub enum Operand {
    Literal(Literal),
    Op(Token),
    Expr(Expression),
}

pub enum Statement {
    Decl(Declaration),
    StmtList(Box<StmtList>),
    PrintStmt(Expression),
    ReturnStmt(Option<Expression>),
    IfStmt(Option<SimpleStmt>, Expression, Box<StmtList>, Box<Statement>),
    SwitchStmt(Option<SimpleStmt>, Option<Expression>, Vec<ExprCaseClause>),
    ForStmt(Option<SimpleStmt>, Option<Expression>, Option<SimpleStmt>),
    BreakStmt,
    ContinueStmt,
    FallthroughStmt,
}

pub enum SimpleStmt {
    ExprStmt(Expression),
    IncDecStmt(Expression, Token),
    Assignment(ExprList, Option<Token>, ExprList),
    ShortValDecl(IdentList, ExprList),
    EmptyStmt,
}

pub enum Declaration {
    Type,
    Var,
    Func(Token, Option<ParamList>, Option<Token>, Box<StmtList>),
    Parameter,
    Array,
    Slice,
    Struct,
}

pub enum Literal {
    Integer(i32),
    Float(f32),
    Rune(char),
    String(String),
    Bool(bool),
    Function(Option<ParamList>, Option<Token>, StmtList)
}

pub struct ExprList {
    expr: Expression,
    next: Expression,
}

pub struct ExprCaseClause {
    expr: ExprList,
    stmt: StmtList,
}

pub struct StmtList {
    stmt: Statement,
    next: Statement,
}

pub struct IdentList {
    idents: Vec<Token>,
}

pub struct ParamList {
    params: Vec<(IdentList, Token)>
}
