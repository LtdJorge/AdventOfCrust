use logos::Logos;

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().unwrap())]
    Numeric(usize),
    #[token("|")]
    Separator,
    #[token("\n")]
    NewLine,
    #[token(",")]
    Comma,
}
