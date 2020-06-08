use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),

    // rules
    EndRule,
    BeginRule,
    DefaultRule,
    EndOfRule,

    // built-in functions
    Print,

    // data types
    Integer(i64),
    Text(String),

    // other 
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,

    // stuff to ignore
    Whitespace,
    Comment,
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r\n]+"# => Token::Whitespace,
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Token::Comment,

    r#"BEGIN \{"# => Token::BeginRule,
    r#"END \{"# => Token::EndRule,
    r#"\{"# => Token::DefaultRule,
    r#"\}\n"# => Token::EndOfRule,
    r#"!mark__bawk__eof!"# => Token::EndOfRule,

    r#"print"# => Token::Print,

    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }

    r#"".*?[^\\]""# => Token::Text(text.trim_matches('"').to_owned()),

    r#"[a-zA-Z_$][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),

    r#"="# => Token::Equals,
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Star,
    r#"/"# => Token::Slash,
    r#"\("# => Token::LParen,
    r#"\)"# => Token::RParen,
    r#"\{"# => Token::LBrace,
    r#"\}"# => Token::RBrace,
    r#";"# => Token::Semi,

    r#"."# => panic!("unexpected character: {}", text),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
