mod ast;
mod js_intl;
mod parser;
mod pattern_syntax;

pub use ast::{Ast, AstElement, Position, Span};
pub use parser::Parser;

#[cfg(test)]
mod tests {
    use crate::ast::*;
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
                    span: Span::new(Position::new(0, 1, 1), Position::new(2, 1, 3))
                },
                AstElement::Argument {
                    value: "b",
                    span: Span::new(Position::new(2, 1, 3), Position::new(5, 1, 6))
                },
                AstElement::Literal {
                    value: " \nc".to_string(),
                    span: Span::new(Position::new(5, 1, 6), Position::new(8, 2, 2))
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
                span: Span::new(Position::new(0, 1, 1), Position::new(12, 1, 13))
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
                span: Span::new(Position::new(0, 1, 1), Position::new(18, 1, 19))
            }])
        )
    }

    /// The apostrophe here does not start a quote because it is not followed by `{` or `}`,
    /// so the `{}` is invalid syntax.
    #[test]
    fn unclosed_quoted_string_2() {
        assert_eq!(
            Parser::new("a 'a {}{}").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "a 'a {}{}".to_string(),
                span: Span::new(Position::new(5, 1, 6), Position::new(7, 1, 8))
            })
        );
    }

    /// The last apostrophe ends the escaping, therefore the last `{}` is invalid syntax.
    #[test]
    fn unclosed_quoted_string_3() {
        assert_eq!(
            Parser::new("a '{a{ {}{}{}}}''' \n {}").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "a '{a{ {}{}{}}}''' \n {}".to_string(),
                span: Span::new(Position::new(21, 2, 2), Position::new(23, 2, 4))
            })
        );
    }

    #[test]
    fn unclosed_quoted_string_4() {
        assert_eq!(
            Parser::new("You have '{count'").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(17, 1, 18))
            }])
        )
    }

    #[test]
    fn unclosed_quoted_string_5() {
        assert_eq!(
            Parser::new("You have '{count").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(16, 1, 17))
            }])
        )
    }

    #[test]
    fn unclosed_quoted_string_6() {
        assert_eq!(
            Parser::new("You have '{count}").parse(),
            Ok(vec![AstElement::Literal {
                value: "You have {count}".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(17, 1, 18))
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

    #[test]
    fn simple_argument_1() {
        assert_eq!(
            Parser::new("My name is {0}").parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "My name is ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(11, 1, 12))
                },
                AstElement::Argument {
                    value: "0",
                    span: Span::new(Position::new(11, 1, 12), Position::new(14, 1, 15))
                }
            ])
        )
    }

    #[test]
    fn simple_argument_2() {
        assert_eq!(
            Parser::new("My name is { name }").parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "My name is ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(11, 1, 12))
                },
                AstElement::Argument {
                    value: "name",
                    span: Span::new(Position::new(11, 1, 12), Position::new(19, 1, 20))
                }
            ])
        )
    }

    #[test]
    fn empty_argument_1() {
        assert_eq!(
            Parser::new("My name is { }").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "My name is { }".to_string(),
                span: Span::new(Position::new(11, 1, 12), Position::new(14, 1, 15))
            })
        )
    }

    #[test]
    fn empty_argument_2() {
        assert_eq!(
            Parser::new("My name is {\n}").parse(),
            Err(Error {
                kind: ErrorKind::EmptyArgument,
                message: "My name is {\n}".to_string(),
                span: Span::new(Position::new(11, 1, 12), Position::new(14, 2, 2))
            })
        )
    }

    #[test]
    fn malformed_argument_1() {
        assert_eq!(
            Parser::new("My name is {0!}").parse(),
            Err(Error {
                kind: ErrorKind::MalformedArgument,
                message: "My name is {0!}".to_string(),
                span: Span::new(Position::new(11, 1, 12), Position::new(13, 1, 14))
            })
        )
    }

    #[test]
    fn unclosed_argument_1() {
        assert_eq!(
            Parser::new("My name is { 0").parse(),
            Err(Error {
                kind: ErrorKind::UnclosedArgumentBrace,
                message: "My name is { 0".to_string(),
                span: Span::new(Position::new(11, 1, 12), Position::new(14, 1, 15))
            })
        )
    }

    #[test]
    fn unclosed_argument_2() {
        assert_eq!(
            Parser::new("My name is { ").parse(),
            Err(Error {
                kind: ErrorKind::UnclosedArgumentBrace,
                message: "My name is { ".to_string(),
                span: Span::new(Position::new(11, 1, 12), Position::new(13, 1, 14))
            })
        )
    }

    #[test]
    fn simple_number_arg_1() {
        assert_eq!(
            Parser::new("I have {numCats, number} cats.").parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "I have ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(7, 1, 8))
                },
                AstElement::Number {
                    value: "numCats",
                    span: Span::new(Position::new(7, 1, 8), Position::new(24, 1, 25)),
                    style: None
                },
                AstElement::Literal {
                    value: " cats.".to_string(),
                    span: Span::new(Position::new(24, 1, 25), Position::new(30, 1, 31))
                },
            ])
        )
    }

    #[test]
    fn simple_date_and_time_arg_1() {
        assert_eq!(
            Parser::new("Your meeting is scheduled for the {dateVal, date} at {timeVal, time}")
                .parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "Your meeting is scheduled for the ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(34, 1, 35))
                },
                AstElement::Date {
                    value: "dateVal",
                    span: Span::new(Position::new(34, 1, 35), Position::new(49, 1, 50)),
                    style: None
                },
                AstElement::Literal {
                    value: " at ".to_string(),
                    span: Span::new(Position::new(49, 1, 50), Position::new(53, 1, 54))
                },
                AstElement::Time {
                    value: "timeVal",
                    span: Span::new(Position::new(53, 1, 54), Position::new(68, 1, 69)),
                    style: None
                },
            ])
        )
    }

    #[test]
    fn invalid_arg_format_1() {
        assert_eq!(
            Parser::new("My name is {0, foo}").parse(),
            Err(Error {
                kind: ErrorKind::InvalidArgumentType,
                message: "My name is {0, foo}".to_string(),
                span: Span::new(Position::new(15, 1, 16), Position::new(18, 1, 19))
            })
        )
    }

    #[test]
    fn expect_arg_format_1() {
        assert_eq!(
            Parser::new("My name is {0, }").parse(),
            Err(Error {
                kind: ErrorKind::ExpectArgumentType,
                message: "My name is {0, }".to_string(),
                span: Span::new(Position::new(15, 1, 16), Position::new(15, 1, 16))
            })
        )
    }

    #[test]
    fn unclosed_number_arg_1() {
        assert_eq!(
            Parser::new("{0, number").parse(),
            Err(Error {
                kind: ErrorKind::UnclosedArgumentBrace,
                message: "{0, number".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(10, 1, 11))
            })
        )
    }

    #[test]
    fn unclosed_number_arg_2() {
        assert_eq!(
            Parser::new("{0, number, percent").parse(),
            Err(Error {
                kind: ErrorKind::UnclosedArgumentBrace,
                message: "{0, number, percent".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(19, 1, 20))
            })
        )
    }

    #[test]
    fn unclosed_number_arg_3() {
        assert_eq!(
            Parser::new("{0, number, ::percent").parse(),
            Err(Error {
                kind: ErrorKind::UnclosedArgumentBrace,
                message: "{0, number, ::percent".to_string(),
                span: Span::new(Position::new(0, 1, 1), Position::new(21, 1, 22))
            })
        )
    }

    #[test]
    fn number_arg_style_1() {
        assert_eq!(
            Parser::new("{0, number, percent}").parse(),
            Ok(vec![AstElement::Number {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(20, 1, 21)),
                style: Some(NumberArgStyle::Style("percent"))
            }])
        )
    }

    #[test]
    fn expect_number_arg_style_1() {
        assert_eq!(
            Parser::new("{0, number, }").parse(),
            Err(Error {
                kind: ErrorKind::ExpectArgumentStyle,
                message: "{0, number, }".to_string(),
                span: Span::new(Position::new(12, 1, 13), Position::new(12, 1, 13))
            })
        )
    }

    #[test]
    fn number_arg_skeleton_1() {
        assert_eq!(
            Parser::new("{0, number, ::percent}").parse(),
            Ok(vec![AstElement::Number {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(22, 1, 23)),
                style: Some(NumberArgStyle::Skeleton(NumberSkeleton {
                    tokens: vec![NumberSkeletonToken { stem: "percent", options: vec![] }],
                    span: Span::new(Position::new(12, 1, 13), Position::new(21, 1, 22)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn number_arg_skeleton_2() {
        assert_eq!(
            Parser::new("{0, number, :: currency/GBP}").parse(),
            Ok(vec![AstElement::Number {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(28, 1, 29)),
                style: Some(NumberArgStyle::Skeleton(NumberSkeleton {
                    tokens: vec![NumberSkeletonToken { stem: "currency", options: vec!["GBP"] }],
                    span: Span::new(Position::new(12, 1, 13), Position::new(27, 1, 28)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn number_arg_skeleton_3() {
        assert_eq!(
            Parser::new("{0, number, ::currency/GBP compact-short}").parse(),
            Ok(vec![AstElement::Number {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(41, 1, 42)),
                style: Some(NumberArgStyle::Skeleton(NumberSkeleton {
                    tokens: vec![
                        NumberSkeletonToken { stem: "currency", options: vec!["GBP"] },
                        NumberSkeletonToken { stem: "compact-short", options: vec![] }
                    ],
                    span: Span::new(Position::new(12, 1, 13), Position::new(40, 1, 41)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn expect_number_arg_skeleton_token_1() {
        assert_eq!(
            Parser::new("{0, number, ::}").parse(),
            Err(Error {
                kind: ErrorKind::ExpectNumberSkeleton,
                message: "{0, number, ::}".to_string(),
                span: Span::new(Position::new(12, 1, 13), Position::new(14, 1, 15))
            })
        )
    }

    #[test]
    fn expect_number_arg_skeleton_token_option_1() {
        assert_eq!(
            Parser::new("{0, number, ::currency/}").parse(),
            Err(Error {
                kind: ErrorKind::InvalidNumberSkeleton,
                message: "{0, number, ::currency/}".to_string(),
                span: Span::new(Position::new(12, 1, 13), Position::new(23, 1, 24),)
            })
        )
    }

    mod number_skeleton_tests {
        use super::*;

        /// Convenient macro to help parametrize tests with number skeleton strings.
        macro_rules! number_skeleton_tests {
            ($($name:ident: $value:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let (skeleton, expected_tokens) = $value;
                        let message = format!("{{0, number, ::{}}}", skeleton);
                        match Parser::new(&message[..]).parse() {
                            Ok(ast_elements) => {
                                match &ast_elements[..] {
                                    [AstElement::Number {
                                        style: Some(NumberArgStyle::Skeleton(NumberSkeleton {
                                            tokens: tokens,
                                            ..
                                        })),
                                        ..
                                    }] => {
                                        assert_eq!(expected_tokens, tokens)
                                    }
                                    _ => panic!("Failed to parse {}", message)
                                }
                            },
                            _ => panic!("Failed to parse {}", message)
                        }
                    }
                )*
            }
        }

        number_skeleton_tests! {
            case_0: ("compact-short currency/GBP", &vec![
                NumberSkeletonToken {
                    stem: "compact-short",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: "currency",
                    options: vec!["GBP"],
                }
            ]),
            case_1: ("@@#", &vec![
                NumberSkeletonToken {
                    stem: "@@#",
                    options: vec![],
                },
            ]),
            case_2: ("currency/CAD unit-width-narrow", &vec![
                NumberSkeletonToken {
                    stem: "currency",
                    options: vec!["CAD"],
                },
                NumberSkeletonToken {
                    stem: "unit-width-narrow",
                    options: vec![],
                }
            ]),
            case_3: ("percent .##", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".##",
                    options: vec![],
                },
            ]),

            // Some percent skeletons
            case_4: ("percent .000*", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".000*",
                    options: vec![],
                },
            ]),
            case_5: ("percent .0###", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".0###",
                    options: vec![],
                },
            ]),
            case_6: ("percent .00/@##", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".00",
                    options: vec!["@##"],
                },
            ]),
            case_7: ("percent .00/@@@", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".00",
                    options: vec!["@@@"],
                },
            ]),
            case_8: ("percent .00/@@@@*", &vec![
                NumberSkeletonToken {
                    stem: "percent",
                    options: vec![],
                },
                NumberSkeletonToken {
                    stem: ".00",
                    options: vec!["@@@@*"],
                },
            ]),

            // Complex currency skeleton
            case_9: ("currency/GBP .00##/@@@ unit-width-full-name", &vec![
                NumberSkeletonToken {
                    stem: "currency",
                    options: vec!["GBP"],
                },
                NumberSkeletonToken {
                    stem: ".00##",
                    options: vec!["@@@"],
                },
                NumberSkeletonToken {
                    stem: "unit-width-full-name",
                    options: vec![],
                },
            ]),

            // Complex unit
            case_10: ("measure-unit/length-meter .00##/@@@ unit-width-full-name", &vec![
                NumberSkeletonToken {
                    stem: "measure-unit",
                    options: vec!["length-meter"],
                },
                NumberSkeletonToken {
                    stem: ".00##",
                    options: vec!["@@@"],
                },
                NumberSkeletonToken {
                    stem: "unit-width-full-name",
                    options: vec![],
                },
            ]),

            // Multiple options
            case_11: ("scientific/+ee/sign-always", &vec![
                NumberSkeletonToken {
                    stem: "scientific",
                    options: vec!["+ee", "sign-always"],
                },
            ]),
        }
    }
}
