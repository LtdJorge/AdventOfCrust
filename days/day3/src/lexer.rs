use logos::{Lexer, Logos};
use std::num::ParseIntError;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error = CustomError)]
pub enum Token {
    #[regex("mul\\([0-9]{1,3},[0-9]{1,3}\\)", mul)]
    Mul((u32, u32)),
    #[cfg(feature = "debug")]
    #[regex(r#"[\u0000-\u007F]"#, |lex| String::from(lex.slice()))]
    Garbage(String),
    #[cfg(not(feature = "debug"))]
    #[regex(r#"[\u0000-\u007F]"#)]
    Garbage,
    #[token("do()")]
    Do,
    #[token("don't()")]
    Dont,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomError {
    message: String,
}

impl From<ParseIntError> for CustomError {
    fn from(_value: ParseIntError) -> Self {
        CustomError {
            message: String::from("ParseIntError"),
        }
    }
}

fn mul(lex: &mut Lexer<Token>) -> Result<(u32, u32), ParseIntError> {
    let end = lex.slice().len();
    let nums = &lex.slice()[4..end - 1];
    let pair = nums.split(',').collect::<Vec<_>>();
    let num1 = pair[0].parse()?;
    let num2 = pair[1].parse()?;
    Ok((num1, num2))
}
