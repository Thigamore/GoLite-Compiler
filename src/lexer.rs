use std::{
    fs,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub enum Token {
    // Keywords
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    Func,
    For,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,
    Print,
    Println,
    Append,
    Len,
    Cap,

    // Operators
    Plus,
    Minus,
    Aster, // \*
    FSlash,
    Percent,
    Amper,
    Or,
    Xor,
    LShift,
    RShift,
    AndNot,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivEqual,
    ModEqual,
    AndEqual,
    OrEqual,
    XorEqual,
    LShiftEqual,
    RShiftEqual,
    AndNotEqual,
    LogAnd,
    LogOr,
    LArrow,
    PlusPlus,
    MinusMinus,
    EqualEqual,
    Less,
    Greater,
    Equal,
    Bang,
    BangEqual,
    LessEqual,
    GreaterEqual,
    Assignment,
    Dots,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Comma,
    Period,
    Semicolon,
    Colon,

    //Literals
    Int(i32),
    Float(f32),
    Rune(char),
    String(String),
    Bool(bool),

    // Error(string, line)
    Error(String, i32),

    // Misc
    Ident(String),
    BlankIdent,
    EOF,
    Empty,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Rune(l0), Self::Rune(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Error(l0, l1), Self::Error(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Break => Self::Break,
            Self::Case => Self::Case,
            Self::Chan => Self::Chan,
            Self::Const => Self::Const,
            Self::Continue => Self::Continue,
            Self::Default => Self::Default,
            Self::Defer => Self::Defer,
            Self::Else => Self::Else,
            Self::Fallthrough => Self::Fallthrough,
            Self::Func => Self::Func,
            Self::For => Self::For,
            Self::Go => Self::Go,
            Self::Goto => Self::Goto,
            Self::If => Self::If,
            Self::Import => Self::Import,
            Self::Interface => Self::Interface,
            Self::Map => Self::Map,
            Self::Package => Self::Package,
            Self::Range => Self::Range,
            Self::Return => Self::Return,
            Self::Select => Self::Select,
            Self::Struct => Self::Struct,
            Self::Switch => Self::Switch,
            Self::Type => Self::Type,
            Self::Var => Self::Var,
            Self::Print => Self::Print,
            Self::Println => Self::Println,
            Self::Append => Self::Append,
            Self::Len => Self::Len,
            Self::Cap => Self::Cap,
            Self::Plus => Self::Plus,
            Self::Minus => Self::Minus,
            Self::Aster => Self::Aster,
            Self::FSlash => Self::FSlash,
            Self::Percent => Self::Percent,
            Self::Amper => Self::Amper,
            Self::Or => Self::Or,
            Self::Xor => Self::Xor,
            Self::LShift => Self::LShift,
            Self::RShift => Self::RShift,
            Self::AndNot => Self::AndNot,
            Self::PlusEqual => Self::PlusEqual,
            Self::MinusEqual => Self::MinusEqual,
            Self::TimesEqual => Self::TimesEqual,
            Self::DivEqual => Self::DivEqual,
            Self::ModEqual => Self::ModEqual,
            Self::AndEqual => Self::AndEqual,
            Self::OrEqual => Self::OrEqual,
            Self::XorEqual => Self::XorEqual,
            Self::LShiftEqual => Self::LShiftEqual,
            Self::RShiftEqual => Self::RShiftEqual,
            Self::AndNotEqual => Self::AndNotEqual,
            Self::LogAnd => Self::LogAnd,
            Self::LogOr => Self::LogOr,
            Self::LArrow => Self::LArrow,
            Self::PlusPlus => Self::PlusPlus,
            Self::MinusMinus => Self::MinusMinus,
            Self::EqualEqual => Self::EqualEqual,
            Self::Less => Self::Less,
            Self::Greater => Self::Greater,
            Self::Equal => Self::Equal,
            Self::Bang => Self::Bang,
            Self::BangEqual => Self::BangEqual,
            Self::LessEqual => Self::LessEqual,
            Self::GreaterEqual => Self::GreaterEqual,
            Self::Assignment => Self::Assignment,
            Self::Dots => Self::Dots,
            Self::LParen => Self::LParen,
            Self::RParen => Self::RParen,
            Self::LBrack => Self::LBrack,
            Self::RBrack => Self::RBrack,
            Self::LBrace => Self::LBrace,
            Self::RBrace => Self::RBrace,
            Self::Comma => Self::Comma,
            Self::Period => Self::Period,
            Self::Semicolon => Self::Semicolon,
            Self::Colon => Self::Colon,
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Float(arg0) => Self::Float(arg0.clone()),
            Self::Rune(arg0) => Self::Rune(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Bool(arg0) => Self::Bool(arg0.clone()),
            Self::Error(arg0, arg1) => Self::Error(arg0.clone(), arg1.clone()),
            Self::Ident(arg0) => Self::Ident(arg0.clone()),
            Self::BlankIdent => Self::BlankIdent,
            Self::EOF => Self::EOF,
            Self::Empty => Self::Empty,
        }
    }
}

impl Token {
    pub fn same_type(&self, other: &Self) -> bool {
        return core::mem::discriminant(self) == core::mem::discriminant(other);
    }
}

pub struct Lexer {
    reader: BufReader<fs::File>,
    chr: char,
    end: bool,
    line: i32,
    prev_token: Token,
    peek_tok: Token,
}

impl Lexer {
    pub fn new(mut reader: BufReader<fs::File>) -> Self {
        let mut buf: [u8; 1] = [0];
        let end = reader.read(&mut buf).expect("Couldn't read file") == 0;
        let mut lex = Self {
            reader,
            end,
            chr: buf[0] as char,
            line: 0,
            prev_token: Token::Empty,
            peek_tok: Token::Empty,
        };
        lex.next_token();
        return lex;
    }

    pub fn eat(&mut self, tok: &Token) {
        if !self.peek().same_type(tok) {
            panic!("Expected: {:?}; Got{:?}", self.peek(), tok);
        }
        self.next_token();
    }

    pub fn peek(&self) -> &Token {
        return &self.peek_tok;
    }

    // Returns the next token in the token stream
    pub fn next_token(&mut self) -> Token {
        let out = self.peek_tok.clone();
        if self.end {
            self.peek_tok = Token::EOF;
            return out;
        }

        println!("{}", self.chr);
        println!("Peek: {:?}", self.peek_tok);
        // Keywords and Identifiers
        if self.chr.is_alphabetic() {
            let name = self.get_ident();
            let tok = self.get_keyword(&name);
            self.prev_token = tok.clone();
            self.skip_whitespace();

            if let Token::Ident(str) = tok {
                if str == "_" {
                    return Token::BlankIdent;
                }
                self.peek_tok = Token::Ident(name);
                return out;
            } else {
                println!("{:?}", tok);
                self.peek_tok = tok;
                return out;
            }
        }
        // Numbers
        else if self.chr.is_numeric() {
            if self.chr == '0' {
                self.next_char();
                match self.chr {
                    'b' | 'B' => {
                        self.next_char();
                        let num = self.get_number(2);
                        self.prev_token = Token::Int(num);
                        self.skip_whitespace();
                        self.peek_tok = Token::Int(num);
                        return out;
                    }
                    'x' | 'X' => {
                        self.next_char();
                        let num = self.get_number(16);
                        self.prev_token = Token::Int(num);
                        self.skip_whitespace();
                        self.peek_tok = Token::Int(num);
                        return out;
                    }
                    'o' | 'O' => {
                        self.next_char();
                        let num = self.get_number(8);
                        self.prev_token = Token::Int(num);
                        self.skip_whitespace();
                        self.peek_tok = Token::Int(num);
                        return out;
                    }
                    _ => {}
                }
            }
            let num = self.get_number(10);
            self.prev_token = Token::Empty;
            self.skip_whitespace();

            if self.chr == '.' {
                self.next_char();
                let decimal = self.get_number(10);
                self.prev_token = Token::Empty;
                self.skip_whitespace();
                self.peek_tok = Token::Float(
                    (num.to_string() + "." + &decimal.to_string())
                        .parse()
                        .unwrap(),
                );
                return out;
            }
            self.peek_tok = Token::Int(num);
            return out;
        }
        // Operators
        else if self.chr.is_ascii_punctuation() {
            // Perhaps negative numbers
            if self.chr == '-' {
                self.next_char();
                if self.chr.is_digit(10) {
                    let num = self.get_number(10);
                    self.prev_token = Token::Empty;
                    self.skip_whitespace();

                    if self.chr == '.' {
                        self.next_char();
                        let decimal = self.get_number(10);
                        self.prev_token = Token::Empty;
                        self.skip_whitespace();
                        self.peek_tok = Token::Float(
                            ("-".to_owned() + &num.to_string() + "." + &decimal.to_string())
                                .parse()
                                .unwrap(),
                        );
                        return out;
                    }
                    self.peek_tok = Token::Int(-1 * num);
                    return out;
                } else {
                    match self.chr {
                        '=' => {
                            self.next_char();
                            self.prev_token = Token::MinusEqual;
                            self.skip_whitespace();
                            self.peek_tok = Token::MinusEqual;
                            return out;
                        }
                        '-' => {
                            self.next_char();
                            self.prev_token = Token::MinusMinus;
                            self.skip_whitespace();
                            self.peek_tok = Token::MinusMinus;
                            return out;
                        }
                        _ => {
                            self.prev_token = Token::Minus;
                            self.skip_whitespace();
                            self.peek_tok = Token::Minus;
                            return out;
                        }
                    }
                }
            }
            // Raw String
            else if self.chr == '`' {
                self.next_char();
                let str = self.get_string();
                self.peek_tok = Token::String(str);
                return out;
            }
            // Strings
            else if self.chr == '"' {
                self.next_char();
                let str = self.get_interpreted_string();
                self.next_char();
                self.prev_token = Token::String("".to_string());
                self.skip_whitespace();
                self.peek_tok = Token::String(str);
                return out;
            }
            // Rune
            else if self.chr == '\'' {
                self.next_char();
                if self.chr == '\\' {
                    self.next_char();
                    let chr = self.get_escape();
                    if chr == '0' {
                        self.peek_tok = Token::Error(chr.to_string(), self.line);
                        return out;
                    }
                    self.next_char();
                    self.prev_token = Token::Rune(' ');
                    self.skip_whitespace();
                    self.peek_tok = Token::Rune(chr);
                    return out;
                }
                let temp = self.chr;
                self.next_char();
                self.prev_token = Token::Rune(' ');
                self.skip_whitespace();
                self.peek_tok = Token::Rune(temp);
                return out;
            }
            self.prev_token = Token::Empty;
            let tok = self.get_operator();
            if let Token::Empty = tok {
            } else {
                self.peek_tok = tok;
            }
            return out;
        }
        // EOF
        else if self.chr == '\0' {
            self.end = true;
            self.peek_tok = Token::EOF;
            return out;
        }
        // whitespace
        else if self.chr.is_whitespace() {
            self.skip_whitespace();
            return self.next_token();
        }
        // Error if ever gets here
        let temp = self.chr;
        let line = self.line;
        self.next_char();
        self.peek_tok = Token::Error(temp.to_string(), line);
        return out;
    }

    fn get_ident(&mut self) -> String {
        let mut str: String = String::from("");
        while self.chr.is_alphanumeric() && !self.end {
            str.push_str(&self.chr.to_string());
            self.next_char();
        }
        return str;
    }

    fn get_number(&mut self, radix: u32) -> i32 {
        let mut num: i32 = 0;
        while self.chr.is_digit(radix) && !self.end {
            num = num * 10 + self.chr.to_digit(10).expect("Not a convertable number") as i32;
            self.next_char();
        }
        return num;
    }

    fn get_string(&mut self) -> String {
        let mut str = String::new();
        while self.chr != '"' {
            str += &self.chr.to_string();
        }
        return str;
    }

    fn get_interpreted_string(&mut self) -> String {
        let mut str = String::new();
        while self.chr != '"' {
            if self.chr == '\\' {
                self.next_char();
                let mut chr = self.get_escape();
                if chr == '0' {
                    chr = ' ';
                }
                // ! Error but don't really know what to do
                str += &chr.to_string();
            } else {
                str += &self.chr.to_string();
            }
            self.next_char();
        }
        return str;
    }

    fn get_escape(&mut self) -> char {
        match self.chr {
            'n' => return '\n',
            '\\' => return '\\',
            't' => return '\t',
            'r' => return '\r',
            '\'' => {
                return '\'';
            }
            '"' => return '"',
            _ => return '0',
        }
    }

    fn get_operator(&mut self) -> Token {
        match self.chr {
            '+' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::PlusEqual;
                    }
                    '+' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::PlusPlus;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Plus;
                    }
                }
            }
            '*' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::TimesEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Aster;
                    }
                }
            }
            '/' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::DivEqual;
                    }
                    '/' => {
                        self.skip_comment();
                        self.next_token();
                        return Token::Empty;
                    }
                    '*' => {
                        self.skip_multiline_comment();
                        self.next_token();
                        return Token::Empty;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::FSlash;
                    }
                }
            }
            '%' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        return Token::ModEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Percent;
                    }
                }
            }
            '&' => {
                self.next_char();
                match self.chr {
                    '^' => {
                        self.next_char();
                        if self.chr == '=' {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::AndNotEqual;
                        }
                        self.skip_whitespace();
                        return Token::AndNot;
                    }
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::AndEqual;
                    }
                    '&' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::LogAnd;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Amper;
                    }
                }
            }
            '|' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::OrEqual;
                    }
                    '|' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::LogOr;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Or;
                    }
                }
            }
            '^' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::XorEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Xor;
                    }
                }
            }
            '<' => {
                self.next_char();
                match self.chr {
                    '<' => {
                        self.next_char();
                        if self.chr == '=' {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::LShiftEqual;
                        }
                        self.skip_whitespace();
                        return Token::LShift;
                    }
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::LessEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Less;
                    }
                }
            }
            '>' => {
                self.next_char();
                match self.chr {
                    '>' => {
                        self.next_char();
                        if self.chr == '=' {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::RShiftEqual;
                        }
                        self.skip_whitespace();
                        return Token::RShift;
                    }
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::GreaterEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Greater;
                    }
                }
            }
            '=' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::EqualEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Equal;
                    }
                }
            }
            '!' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::BangEqual;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Bang;
                    }
                }
            }
            ':' => {
                self.next_char();
                match self.chr {
                    '=' => {
                        self.next_char();
                        self.skip_whitespace();
                        return Token::Assignment;
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Colon;
                    }
                }
            }
            '.' => {
                self.next_char();
                match self.chr {
                    '.' => {
                        let line = self.line;
                        self.next_char();
                        if self.chr == '.' {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::Dots;
                        } else {
                            return Token::Error("..".to_string(), line);
                        }
                    }
                    _ => {
                        self.skip_whitespace();
                        return Token::Period;
                    }
                }
            }
            ',' => {
                self.next_char();
                self.skip_whitespace();
                return Token::Comma;
            }
            '(' => {
                self.next_char();
                self.skip_whitespace();
                return Token::LParen;
            }
            ')' => {
                self.next_char();
                self.skip_whitespace();
                return Token::RParen;
            }
            '[' => {
                self.next_char();
                self.skip_whitespace();
                return Token::LBrack;
            }
            ']' => {
                self.next_char();
                self.skip_whitespace();
                return Token::RBrack;
            }
            '{' => {
                self.next_char();
                self.skip_whitespace();
                return Token::LBrace;
            }
            '}' => {
                self.next_char();
                self.skip_whitespace();
                return Token::RBrace;
            }
            ';' => {
                self.next_char();
                self.skip_whitespace();
                return Token::Semicolon;
            }
            _ => {
                let temp = self.chr;
                let line = self.line;
                self.next_char();
                self.skip_whitespace();
                return Token::Error(temp.to_string(), line);
            }
        }
    }

    fn get_keyword(&mut self, str: &str) -> Token {
        match str {
            "break" => Token::Break,
            "case" => Token::Case,
            "continue" => Token::Continue,
            "default" => Token::Default,
            "defer" => Token::Defer,
            "else" => Token::Else,
            "fallthrough" => Token::Fallthrough,
            "func" => Token::Func,
            "for" => Token::For,
            "go" => Token::Go,
            "goto" => Token::Goto,
            "if" => Token::If,
            "import" => Token::Import,
            "interface" => Token::Interface,
            "map" => Token::Map,
            "package" => Token::Package,
            "range" => Token::Range,
            "return" => Token::Return,
            "select" => Token::Select,
            "struct" => Token::Struct,
            "switch" => Token::Switch,
            "type" => Token::Type,
            "var" => Token::Var,
            "print" => Token::Print,
            "println" => Token::Println,
            "append" => Token::Append,
            "len" => Token::Len,
            "cap" => Token::Cap,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Ident("".to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.chr.is_whitespace() {
            if self.chr == '\n' {
                self.line += 1;
                match self.prev_token {
                    Token::Ident(_)
                    | Token::Int(_)
                    | Token::Float(_)
                    | Token::Rune(_)
                    | Token::String(_)
                    | Token::Break
                    | Token::Continue
                    | Token::Fallthrough
                    | Token::Return
                    | Token::PlusPlus
                    | Token::MinusMinus
                    | Token::RParen
                    | Token::RBrace
                    | Token::RBrack => {
                        self.chr = ';';
                        return;
                    }
                    _ => {}
                }
            }
            self.next_char();
        }
    }

    fn skip_comment(&mut self) {
        while self.chr != '\n' && !self.end {
            self.next_char();
        }
        self.line += 1;
    }

    fn skip_multiline_comment(&mut self) {
        self.next_char();
        let mut past = self.chr;
        self.next_char();
        while self.chr != '/' && past != '*' {
            if self.chr == '\n' {
                self.line += 1;
            }
            past = self.chr;
            self.next_char();
        }
        self.next_char();
    }

    fn next_char(&mut self) {
        let mut buf = [0];
        self.end = self.reader.read(&mut buf).expect("Couldn't read file") == 0;
        self.chr = buf[0] as char;
    }
}
