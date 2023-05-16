use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[error]
    Error,
}
