mod ast;
mod js_intl;
mod parser;

pub use ast::{Ast, AstElement, Position, Span};
pub use parser::Parser;

#[cfg(test)]
mod tests {
    use crate::ast::{AstElement, Error, ErrorKind, Position, Span};
    use crate::parser::Parser;

    #[test]
    fn trivial_1() {
        assert_eq!(
            Parser::new("a").parse(),
            Ok(vec![AstElement::Literal {
                value: "a".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(1, 1, 2))
            }])
        );
    }

    #[test]
    fn trivial_2() {
        assert_eq!(
            Parser::new("中文").parse(),
            Ok(vec![AstElement::Literal {
                value: "中文".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(6, 1, 3))
            }])
        );
    }

    #[test]
    fn basic_argument_1() {
        assert_eq!(
            Parser::new("{a}").parse(),
            Ok(vec![AstElement::Argument {
                value: "a",
                span: Span::new(Position::new(0, 1, 1), Position::new(3, 1, 4))
            }])
        );
    }

    #[test]
    fn basic_argument_2() {
        assert_eq!(
            Parser::new("a {b} \nc").parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "a ".to_string(),
                    span: Span::new(
                        Position::new(0, 1, 1),
                        Position::new(2, 1, 3)
                    )
                },
                AstElement::Argument {
                    value: "b",
                    span: Span::new(
                        Position::new(2, 1, 3),
                        Position::new(5, 1, 6)
                    )
                },
                AstElement::Literal {
                    value: " \nc".to_string(),
                    span: Span::new(
                        Position::new(5, 1, 6),
                        Position::new(8, 2, 2)
                    )
                },
            ])
        );
    }

    #[test]
    fn unescaped_string_literal_1() {
        assert_eq!(
            Parser::new("}").parse(),
            Ok(vec![AstElement::Literal {
                value: "}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(1, 1, 2))
            }])
        )
    }

    #[test]
    fn double_apostrophes_1() {
        assert_eq!(
            Parser::new("a''b").parse(),
            Ok(vec![AstElement::Literal {
                value: "a'b".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(4, 1, 5))
            }])
        )
    }

    #[test]
    fn quoted_string_1() {
        assert_eq!(
            Parser::new("'{a''b}'").parse(),
            Ok(vec![AstElement::Literal {
                value: "{a'b}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(8, 1, 9))
            }])
        )
    }

    #[test]
    fn quoted_string_2() {
        assert_eq!(
            Parser::new("'}a''b{'").parse(),
            Ok(vec![AstElement::Literal {
                value: "}a'b{".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(8, 1, 9))
            }])
        )
    }

    #[test]
    fn quoted_string_3() {
        assert_eq!(
            Parser::new("aaa'{'").parse(),
            Ok(vec![AstElement::Literal {
                value: "aaa{".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(6, 1, 7))
            }])
        )
    }

    #[test]
    fn quoted_string_4() {
        assert_eq!(
            Parser::new("aaa'}'").parse(),
            Ok(vec![AstElement::Literal {
                value: "aaa}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(6, 1, 7))
            }])
        )
    }

    #[test]
    fn not_quoted_string_1() {
        assert_eq!(
            Parser::new("'aa''b'").parse(),
            Ok(vec![AstElement::Literal {
                value: "'aa'b'".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(7, 1, 8))
            }])
        )
    }

    #[test]
    fn not_quoted_string_2() {
        assert_eq!(
            Parser::new("I don't know").parse(),
            Ok(vec![AstElement::Literal {
                value: "I don't know".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(12, 1, 13)
                )
            }])
        )
    }

    /// Substring starting at th apostrophe are all escaped because the quote did not close
    #[test]
    fn unclosed_quoted_string_1() {
        assert_eq!(
            Parser::new("a '{a{ {}{}{} ''bb").parse(),
            Ok(vec![AstElement::Literal {
                value: "a {a{ {}{}{} 'bb".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(18, 1, 19)
                )
            }])
        )
    }

    /// The apostrophe here does not start a quote because it is not followed by `{` or `}`,
    /// so the `{}` is invalid syntax.
    #[ignore]
    #[test]
    fn unclosed_quoted_string_2() {
        assert_eq!(
            Parser::new("a 'a {}{}").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "a 'a {}{}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(0, 1, 1))
            })
        );
    }

    /// The last apostrophe ends the escaping, therefore the last `{}` is invalid syntax.
    #[ignore]
    #[test]
    fn unclosed_quoted_string_3() {
        assert_eq!(
            Parser::new("a '{a{ {}{}{}}}''' \n {}").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "a '{a{ {}{}{}}}''' \n {}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(0, 1, 1))
            })
        );
    }

    #[test]
    fn unclosed_quoted_string_4() {
        assert_eq!(
            Parser::new("You have '{count'").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(17, 1, 18)
                )
            }])
        )
    }

    #[test]
    fn unclosed_quoted_string_5() {
        assert_eq!(
            Parser::new("You have '{count").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(16, 1, 17)
                )
            }])
        )
    }

    #[test]
    fn unclosed_quoted_string_6() {
        assert_eq!(
            Parser::new("You have '{count}").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count}".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(17, 1, 18)
                )
            }])
        )
    }

    #[ignore]
    #[test]
    fn quoted_pound_sign_1() {
        assert_eq!(
            Parser::new("You {count, plural, one {worked for '#' hour} other {worked for '#' hours}} today.").parse(),
            Ok(vec![AstElement::Literal {
                value: "You {count, plural, one {worked for '#' hour} other {worked for '#' hours}} today.".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(1, 1, 1)
                )
            }])
        )
    }

    #[ignore]
    #[test]
    fn quoted_pound_sign_2() {
        // '# hour} other {worked for ' is quoted.
        assert_eq!(
            Parser::new("You {count, plural, one {worked for '# hour} other {worked for '# hours}} today.car").parse(),
            Ok(vec![AstElement::Literal {
                value: "You {count, plural, one {worked for '# hour} other {worked for '# hours}} today.".to_string(),
                span: Span::new(
                    Position::new(0, 1, 1),
                    Position::new(1, 1, 1)
                )
            }])
        )
    }
}
