use std::{collections::HashMap, string};

use super::Token;

pub struct AST {
    package: String,
    main: Declaration,
    func_env: HashMap<Token, Literal>,
    val_env: HashMap<Token, Literal>,
    type_env: HashMap<Token, Literal>,
}

pub enum Expression {
    Unary(Operator, Box<Expression>),
    Binary(Box<Expression>, Operator, Box<Expression>),
    PrimaryExpr(Box<PrimaryExpr>),
}

pub enum PrimaryExpr {
    Append(Expression, Expression),
    Length(Expression),
    Capacity(Expression),
    Operand(Operand),
    Selector(Box<PrimaryExpr>, String),
    Index(Box<PrimaryExpr>, Expression),
    Call(Option<Box<PrimaryExpr>>, Option<Type>, Option<ExprList>), // Used by conversion and call
}

pub enum Operand {
    Literal(Literal),
    Op(String),
    Expr(Expression),
}

pub enum Statement {
    SimpleStmt(SimpleStmt),
    Decl(Declaration),
    StmtList(Box<StmtList>),
    PrintStmt(PrintType, Option<ExprList>),
    ReturnStmt(Option<Expression>),
    IfStmt(Option<SimpleStmt>, Expression, Box<StmtList>, Option<Box<Statement>>),
    SwitchStmt(Option<SimpleStmt>, Option<Expression>, Vec<ExprCaseClause>),
    ForStmt(Option<ForClause>, StmtList),
    BreakStmt,
    ContinueStmt,
    FallthroughStmt,
}

pub enum SimpleStmt {
    ExprStmt(Expression),
    IncDecStmt(Expression, IncDecType),
    Assignment(ExprList, Option<Operator>, ExprList),
    ShortValDecl(Vec<String>, ExprList),
    EmptyStmt,
}

pub enum Declaration {
    Type(Vec<(String, Type)>),
    Var(Vec<VarSpec>),
    Parameter,
    Array,
    Slice,
    Struct,
}

pub enum Literal {
    Integer(i32),
    Float(f64),
    Rune(char),
    String(String),
    Bool(bool),
    Function(Option<ParamList>, Option<Token>, StmtList)
}

#[derive(Clone, Debug)]
pub enum Type {
    Struct(String),
    Array(i32, Box<Type>),
    Slice(Box<Type>),
    Integer,
    Float,
    String,
    Bool,
    Rune,
}

pub enum TypeDecl {
    Struct(HashMap<String, Type>),
    Primitive(Type)
}

pub enum PrintType {
    Print,
    Println
}

pub enum IncDecType {
    Inc,
    Dec
}

pub enum Operator {
    Plus,
    Times,
    Minus,
    Divide,
    Mod,
    RShift,
    LShift,
    Xor,
    AndNot,
    And,
    Not,
    Or,
    LogAnd,
    LogOr,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    NotEqual,
}



pub struct ExprList {
    pub exprs: Vec<Expression>
}

pub struct ExprCaseClause {
    pub expr: Option<ExprList>,
    pub stmt: StmtList,
}


pub struct StmtList {
    pub stmts: Vec<Statement>
}

pub struct ParamList {
    pub params: Vec<(Vec<String>, Type)>
}

pub struct FuncLiteral {
    pub params: Option<ParamList>,
    pub ret: Option<Type>,
    pub body: StmtList,
}

// Can be a for clause or just a plain expr
pub struct ForClause {
    pub stmt1: Option<SimpleStmt>,
    pub expr: Option<Expression>,
    pub stmt2: Option<SimpleStmt>
}

pub struct VarSpec {
    pub ident_list: Vec<String>,
    pub var_type: Option<Type>,
    pub expr_list: Option<ExprList>
}

pub enum VarValue {
    Int(i32),
    Float(f32),
    Rune(char),
    String(String),
    Bool(bool),
    Struct(HashMap<String, VarValue>),
    Array(Vec<VarValue>),
}

impl AST {
    pub fn print() {

    }
}