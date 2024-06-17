use std::{
    fs,
    io::{BufReader, Read},
};

#[derive(Debug, PartialEq)]
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

    // Error(string, line)
    Error(String, i32),

    // Misc
    Ident(String),
    BlankIdent,
    EOF,
    Empty
}

pub struct Lexer {
    reader: BufReader<fs::File>,
    chr: char,
    end: bool,
    line: i32,
    prev_token: Token,
}

impl Lexer {
    pub fn new(mut reader: BufReader<fs::File>) -> Self {
        let mut buf: [u8; 1] = [0];
        let end = reader.read(&mut buf).expect("Couldn't read file") == 0;
        return Self {
            reader,
            end,
            chr: buf[0] as char,
            line: 0,
            prev_token: Token::Empty
        };
    }

    // Returns the next token in the token stream
    pub fn next_token(&mut self) -> Token {
        if self.end {
            return Token::EOF;
        }

        // Keywords and Identifiers
        if self.chr.is_alphabetic() {
           let name;
            if self.chr == 'r' {
                self.next_char();
                if self.chr == '"' {
                    let temp = self.get_string();
                    self.next_char();
                    self.skip_whitespace();
                    return Token::String(temp);
                } else {
                    name = "r".to_owned() + &self.get_ident();
                }
            } else {
                name = self.get_ident();
            }
            let tok = self.get_keyword(&name);
            let prev_line = self.line;
            self.skip_whitespace();

            if let Token::Ident(str) = tok {
                if str == "_" {
                    return Token::BlankIdent;
                }
                return Token::Ident(name);
            } else {
                return tok;
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
                        self.skip_whitespace();
                        return Token::Int(num);
                    }
                    'x' | 'X' => {
                        self.next_char();
                        let num = self.get_number(16);
                        self.skip_whitespace();
                        return Token::Int(num);
                    }
                    'o' | 'O' => {
                        self.next_char();
                        let num = self.get_number(8);
                        self.skip_whitespace();
                        return Token::Int(num);
                    }
                    _ => {}
                }
            }
            let num = self.get_number(10);
            self.skip_whitespace();

            if self.chr == '.' {
                self.next_char();
                let decimal = self.get_number(10);
                self.skip_whitespace();
                return Token::Float(
                    (num.to_string() + "." + &decimal.to_string())
                        .parse()
                        .unwrap(),
                );
            }
            return Token::Int(num);
        }
        // Operators
        else if self.chr.is_ascii_punctuation() {
            // Perhaps negative numbers
            if self.chr == '-' {
                self.next_char();
                if self.chr.is_digit(10) {
                    let num = self.get_number(10);
                    self.skip_whitespace();

                    if self.chr == '.' {
                        self.next_char();
                        let decimal = self.get_number(10);
                        self.skip_whitespace();
                        return Token::Float(
                            ("-".to_owned() + &num.to_string()+ "." + &decimal.to_string())
                                .parse()
                                .unwrap(),
                        );
                    }
                    return Token::Int(-1 * num);
                } else {
                    match self.chr {
                        '=' => {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::MinusEqual;
                        }
                        '-' => {
                            self.next_char();
                            self.skip_whitespace();
                            return Token::MinusMinus;
                        }
                        _ => {
                            self.skip_whitespace();
                            return Token::Minus;
                        }
                    }
                }
            }
            return self.get_operator();
        }
        // Strings
        else if self.chr == '"' {
            self.next_char();
            let str = self.get_interpreted_string();
            self.next_char();
            self.skip_whitespace();
            return Token::String(str);
        }
        // Rune
        else if self.chr == '\'' {
            self.next_char();
            if self.chr == '\\' {
                self.next_char();
                let chr = self.get_escape();
                if chr == '0' {
                    return Token::Error(chr.to_string(), self.line);
                }
                self.next_char();
                self.skip_whitespace();
                return Token::Rune(chr);
            }
            let temp = self.chr;
            self.next_char();
            self.skip_whitespace();
            return Token::Rune(temp);
        }
        // EOF
        else if self.chr == '\0' {
            self.end = true;
            return Token::EOF;
        }
        // whitespace
        else if self.chr == ' ' || self.chr == '\n' {
            self.skip_whitespace();
            return self.next_token();
        }
        // Error if ever gets here
        let temp = self.chr;
        let line = self.line;
        self.next_char();
        return Token::Error(temp.to_string(), line);
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
                    'n' => {
                        return '\n'
                    }
                    '\\' => {
                        return '\\'
                    }
                    't' => {
                        return '\t'
                    }
                    'r' => {
                        return '\r'
                    }
                    '\'' => {
                        return '\'';
                    }
                    '"' => {
                        return '"'
                    }
                    _ => {
                        return '0'
                    }
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
                        return self.next_token();
                    }
                    '*' => {
                        self.skip_multiline_comment();
                        return self.next_token();
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
            _ => Token::Ident("".to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.chr.is_whitespace() {
            if self.chr == '\n' {
                self.line += 1;
                match self.prev_token {
                    Token::Ident(_) | Token::Int(_) | Token::Float(_) | Token::Rune(_) | Token::String(_) | Token::Break | Token::Continue | Token::Fallthrough | Token::Return | Token::PlusPlus | Token::MinusMinus | Token::RParen | Token::RBrace | Token::RBrace => {
                        self.chr = ';';
                        return
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
