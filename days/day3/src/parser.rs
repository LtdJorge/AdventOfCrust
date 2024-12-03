use crate::lexer::Token;
use chumsky::extra::State;
use chumsky::input::MapExtra;
use chumsky::{
    prelude::{any, choice, group, just},
    IterParser, Parser,
};

pub enum ParserStatus {
    Enabled,
    Disabled,
}

pub struct ParserState {
    pub(crate) status: ParserStatus,
}

pub fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<u32>, State<ParserState>> {
    let muls = any()
        .filter(|token: &Token| matches!(token, Token::Mul(_)))
        .map_with(|token, e| match token {
            Token::Mul((left, right)) => {
                let state: &mut ParserState = e.state();
                match state.status {
                    ParserStatus::Enabled => left * right,
                    ParserStatus::Disabled => 0,
                }
            }
            _ => unreachable!(),
        });
    let dos = group((
        just(Token::Do).map_with(|_, e: &mut MapExtra<&'a [Token], State<ParserState>>| {
            e.state().status = ParserStatus::Enabled;
        }),
        muls,
    ));
    let donts = group((
        just(Token::Dont).map_with(|_, e: &mut MapExtra<&'a [Token], State<ParserState>>| {
            e.state().status = ParserStatus::Disabled;
        }),
        muls,
    ));

    choice((dos.map(|(_, mul)| mul), donts.map(|(_, _)| 0), muls))
        .repeated()
        .collect::<Vec<_>>()
}
