use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct RawCall<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

fn sep(input: &str) -> IResult<&str, &str> {
    alt((tag(","), tag("("), tag(")")))(input)
}

fn sep_spaced(input: &str) -> IResult<&str, &str> {
    delimited(space0, sep, space0)(input)
}

fn argument(input: &str) -> IResult<&str, &str> {
    is_not(",()")(input)
}

fn argument_spaced(input: &str) -> IResult<&str, &str> {
    map(argument, str::trim)(input)
}

fn argument_maybe(input: &str) -> IResult<&str, &str> {
    map(opt(argument_spaced), Option::unwrap_or_default)(input)
}

fn sep_argument(input: &str) -> IResult<&str, &str> {
    preceded(sep_spaced, argument_maybe)(input)
}

fn arguments(input: &str) -> IResult<&str, Vec<&str>> {
    many0(sep_argument)(input)
}

fn call_name(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn call(input: &str) -> IResult<&str, RawCall> {
    map(tuple((call_name, arguments)), |(name, args)| RawCall {
        name,
        args,
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::RawCall;

    #[test]
    fn sep() {
        assert_eq!(super::sep(", "), Ok((" ", ",")));
        assert_eq!(super::sep(")"), Ok(("", ")")));
        assert_eq!(super::sep("("), Ok(("", "(")));
    }

    #[test]
    fn sep_spaced() {
        assert_eq!(super::sep_spaced(" , "), Ok(("", ",")));
        assert_eq!(super::sep_spaced(" )"), Ok(("", ")")));
        assert_eq!(super::sep_spaced("("), Ok(("", "(")));
    }

    #[test]
    fn argument() {
        assert_eq!(super::argument("  hi  ,"), Ok((",", "  hi  ")));
        assert_eq!(super::argument("  hi  ("), Ok(("(", "  hi  ")));
        assert_eq!(super::argument("  hi  )"), Ok((")", "  hi  ")));
    }

    #[test]
    fn argument_spaced() {
        assert_eq!(super::argument_spaced("  hi  ,"), Ok((",", "hi")));
        assert_eq!(super::argument_spaced("  hi  ("), Ok(("(", "hi")));
        assert_eq!(super::argument_spaced("  hi  )"), Ok((")", "hi")));
    }

    #[test]
    fn argument_maybe() {
        assert_eq!(super::argument_maybe("  hi  ,"), Ok((",", "hi")));
        assert_eq!(super::argument_maybe("  hi  ("), Ok(("(", "hi")));
        assert_eq!(super::argument_maybe("  hi  )"), Ok((")", "hi")));
        assert_eq!(super::argument_maybe(""), Ok(("", "")));
    }

    #[test]
    fn sep_argument() {
        assert_eq!(super::sep_argument(")  hi  ,"), Ok((",", "hi")));
        assert_eq!(super::sep_argument("(  hi  ("), Ok(("(", "hi")));
        assert_eq!(super::sep_argument(",  hi  )"), Ok((")", "hi")));
    }

    #[test]
    fn arguments() {
        assert_eq!(
            super::arguments(")  hi  ,  hi  (  hi  )"),
            Ok(("", vec!["hi", "hi", "hi", ""]))
        );
    }

    #[test]
    fn call_name() {
        assert_eq!(super::call_name("asdfasdf"), Ok(("", "asdfasdf")));
        assert_eq!(super::call_name("asdfasdf2"), Ok(("", "asdfasdf2")));
        assert_eq!(super::call_name("asdfasdf2("), Ok(("(", "asdfasdf2")));
        assert!(super::call_name("").is_err());
    }

    #[test]
    fn call() {
        assert_eq!(
            super::call("say(2)"),
            Ok((
                "",
                RawCall {
                    name: "say",
                    args: vec!["2", ""],
                }
            ))
        );
        assert_eq!(
            super::call("endtext"),
            Ok((
                "",
                RawCall {
                    name: "endtext",
                    args: vec![],
                }
            ))
        );
        assert_eq!(
            super::call("createcrewman,0,0, red,0,followplayer"),
            Ok((
                "",
                RawCall {
                    name: "createcrewman",
                    args: vec!["0", "0", "red", "0", "followplayer"],
                }
            ))
        );
    }
}
