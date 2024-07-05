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
    Float(f32),
    Rune(char),
    String(String),
    Bool(bool),
    Function(Option<ParamList>, Option<Token>, StmtList)
}

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
}

pub struct ExprList {
    expr: Expression,
    next: Expression,
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
    pub var_type: Type,
    pub expr_list: Option<ExprList>
}