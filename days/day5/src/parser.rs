use crate::{
    lexer::Token,
    types::{PageOrderingRule, Update},
};
use chumsky::{
    error::Rich,
    extra::Full,
    prelude::{any, group, just},
    IterParser, Parser,
};

pub fn parser<'a>(
) -> impl Parser<'a, &'a [Token], (Vec<PageOrderingRule>, Vec<Update>), Full<Rich<'a, Token>, (), ()>>
{
    let num = any()
        .filter(|token| matches!(token, Token::Numeric(_)))
        .map(|token: Token| match token {
            Token::Numeric(num) => num,
            _ => unreachable!(),
        });

    let page_ordering = group((num, just(Token::Separator), num, just(Token::NewLine)))
        .map(|(left, _, right, _)| PageOrderingRule { left, right });

    let update = group((
        any()
            .filter(|token: &Token| matches!(token, Token::Numeric(_) | Token::Comma))
            .map(|token| match token {
                Token::Numeric(num) => Some(num),
                Token::Comma => None,
                _ => unreachable!(),
            })
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|vec| Update {
                list: vec.iter().filter_map(|num| *num).collect::<Vec<_>>(),
            }),
        just(Token::NewLine),
    ))
    .map(|(vec, _)| vec);

    group((
        page_ordering.repeated().collect::<Vec<_>>(),
        just(Token::NewLine),
        update.repeated().collect::<Vec<_>>(),
    ))
    .map(|(line1, _, line2)| (line1, line2))
}
