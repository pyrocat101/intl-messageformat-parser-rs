mod ast;
mod js_intl;
mod parser;
mod pattern_syntax;

pub use ast::{Ast, AstElement, Position, Span};
pub use parser::Parser;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
extern crate indoc;

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

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

    // `#` are escaped in the plural argument.
    #[test]
    fn quoted_pound_sign_1() {
        assert_eq!(
            Parser::new("You {count, plural, one {worked for '#' hour} other {worked for '#' hours}} today.").parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "You ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(4, 1, 5))
                },
                AstElement::Plural {
                    value: "count",
                    plural_type: PluralType::Cardinal,
                    span: Span::new(Position::new(4, 1, 5), Position::new(75, 1, 76)),
                    offset: 0,
                    options: PluralOrSelectOptions(vec![
                        ("one", PluralOrSelectOption {
                            value: vec![
                                AstElement::Literal {
                                    value: "worked for # hour".to_string(),
                                    span: Span::new(Position::new(25, 1, 26), Position::new(44, 1, 45))
                                },
                            ],
                            span: Span::new(Position::new(24, 1, 25), Position::new(45, 1, 46)),
                        }),
                        ("other", PluralOrSelectOption {
                            value: vec![
                                AstElement::Literal {
                                    value: "worked for # hours".to_string(),
                                    span: Span::new(Position::new(53, 1, 54), Position::new(73, 1, 74))
                                },
                            ],
                            span: Span::new(Position::new(52, 1, 53), Position::new(74, 1, 75))
                        })
                    ])
                },
                AstElement::Literal {
                    value: " today.".to_string(),
                    span: Span::new(Position::new(75, 1, 76), Position::new(82, 1, 83))
                }
            ])
        )
    }

    #[test]
    fn quoted_pound_sign_2() {
        // '# hour} other {worked for ' is quoted.
        assert_eq!(
            Parser::new(
                "You {count, plural, one {worked for '# hour} other {worked for '# hours}} today."
            )
            .parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "You ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(4, 1, 5)),
                },
                AstElement::Plural {
                    value: "count",
                    plural_type: PluralType::Cardinal,
                    span: Span::new(Position::new(4, 1, 5), Position::new(73, 1, 74)),
                    offset: 0,
                    options: PluralOrSelectOptions(vec![(
                        "one",
                        PluralOrSelectOption {
                            value: vec![
                                AstElement::Literal {
                                    value: "worked for # hour} other {worked for ".to_string(),
                                    span: Span::new(
                                        Position::new(25, 1, 26),
                                        Position::new(64, 1, 65)
                                    ),
                                },
                                AstElement::Pound(Span::new(
                                    Position::new(64, 1, 65),
                                    Position::new(65, 1, 66)
                                ),),
                                AstElement::Literal {
                                    value: " hours".to_string(),
                                    span: Span::new(
                                        Position::new(65, 1, 66),
                                        Position::new(71, 1, 72)
                                    ),
                                },
                            ],
                            span: Span::new(Position::new(24, 1, 25), Position::new(72, 1, 73)),
                        },
                    )]),
                },
                AstElement::Literal {
                    value: " today.".to_string(),
                    span: Span::new(Position::new(73, 1, 74), Position::new(80, 1, 81)),
                },
            ])
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

    #[test]
    fn date_arg_skeleton_1() {
        assert_eq!(
            Parser::new("{0, date, ::yyyy.MM.dd G 'at' HH:mm:ss vvvv}").parse(),
            Ok(vec![AstElement::Date {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(44, 1, 45)),
                style: Some(DateTimeArgStyle::Skeleton(DateTimeSkeleton {
                    pattern: "yyyy.MM.dd G 'at' HH:mm:ss vvvv",
                    span: Span::new(Position::new(10, 1, 11), Position::new(43, 1, 44)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn date_arg_skeleton_2() {
        assert_eq!(
            Parser::new("{0, date, ::EEE, MMM d, ''yy}").parse(),
            Ok(vec![AstElement::Date {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(29, 1, 30)),
                style: Some(DateTimeArgStyle::Skeleton(DateTimeSkeleton {
                    pattern: "EEE, MMM d, ''yy",
                    span: Span::new(Position::new(10, 1, 11), Position::new(28, 1, 29)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn date_arg_skeleton_3() {
        assert_eq!(
            Parser::new("{0, date, ::h:mm a}").parse(),
            Ok(vec![AstElement::Date {
                value: "0",
                span: Span::new(Position::new(0, 1, 1), Position::new(19, 1, 20)),
                style: Some(DateTimeArgStyle::Skeleton(DateTimeSkeleton {
                    pattern: "h:mm a",
                    span: Span::new(Position::new(10, 1, 11), Position::new(18, 1, 19)),
                    parsed_options: None,
                }))
            }])
        )
    }

    #[test]
    fn duplicate_plural_selectors() {
        assert_eq!(
            Parser::new("You have {count, plural, one {# hot dog} one {# hamburger} one {# sandwich} other {# snacks}} in your lunch bag.").parse(),
            Err(Error {
                kind: ErrorKind::DuplicatePluralArgumentSelector,
                message: "You have {count, plural, one {# hot dog} one {# hamburger} one {# sandwich} other {# snacks}} in your lunch bag.".to_string(),
                span: Span::new(Position::new(41, 1, 42), Position::new(44, 1, 45))
            })
        )
    }

    #[test]
    fn duplicate_select_selectors() {
        assert_eq!(
            Parser::new("You have {count, select, one {# hot dog} one {# hamburger} one {# sandwich} other {# snacks}} in your lunch bag.").parse(),
            Err(Error {
                kind: ErrorKind::DuplicateSelectArgumentSelector,
                message: "You have {count, select, one {# hot dog} one {# hamburger} one {# sandwich} other {# snacks}} in your lunch bag.".to_string(),
                span: Span::new(Position::new(41, 1, 42), Position::new(44, 1, 45))
            })
        )
    }

    #[test]
    fn treat_unicode_nbsp_as_whitespace() {
        assert_eq!(
            Parser::new(indoc! {"{gender, select,
                \u{00a0}male {
                    {He}}
                \u{00a0}female {
                    {She}}
                \u{00a0}other{
                    {They}}}
            "})
            .parse(),
            Ok(vec![
                AstElement::Select {
                    value: "gender",
                    span: Span::new(Position::new(0, 1, 1), Position::new(79, 7, 13)),
                    options: PluralOrSelectOptions(vec![
                        (
                            "male",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Literal {
                                        value: "\n    ".to_string(),
                                        span: Span::new(
                                            Position::new(25, 2, 8),
                                            Position::new(30, 3, 5)
                                        ),
                                    },
                                    AstElement::Argument {
                                        value: "He",
                                        span: Span::new(
                                            Position::new(30, 3, 5),
                                            Position::new(34, 3, 9)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(24, 2, 7), Position::new(35, 3, 10)),
                            },
                        ),
                        (
                            "female",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Literal {
                                        value: "\n    ".to_string(),
                                        span: Span::new(
                                            Position::new(46, 4, 10),
                                            Position::new(51, 5, 5)
                                        ),
                                    },
                                    AstElement::Argument {
                                        value: "She",
                                        span: Span::new(
                                            Position::new(51, 5, 5),
                                            Position::new(56, 5, 10)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(45, 4, 9), Position::new(57, 5, 11)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Literal {
                                        value: "\n    ".to_string(),
                                        span: Span::new(
                                            Position::new(66, 6, 8),
                                            Position::new(71, 7, 5)
                                        ),
                                    },
                                    AstElement::Argument {
                                        value: "They",
                                        span: Span::new(
                                            Position::new(71, 7, 5),
                                            Position::new(77, 7, 11)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(65, 6, 7), Position::new(78, 7, 12)),
                            },
                        ),
                    ]),
                },
                AstElement::Literal {
                    value: "\n".to_string(),
                    span: Span::new(Position::new(79, 7, 13), Position::new(80, 8, 1)),
                },
            ])
        )
    }

    #[test]
    fn plural_arg_1() {
        assert_eq!(
            Parser::new(indoc! {"\
            Cart: {itemCount} {itemCount, plural,
              one {item}
              other {items}
            }"})
            .parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "Cart: ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(6, 1, 7)),
                },
                AstElement::Argument {
                    value: "itemCount",
                    span: Span::new(Position::new(6, 1, 7), Position::new(17, 1, 18)),
                },
                AstElement::Literal {
                    value: " ".to_string(),
                    span: Span::new(Position::new(17, 1, 18), Position::new(18, 1, 19)),
                },
                AstElement::Plural {
                    value: "itemCount",
                    plural_type: PluralType::Cardinal,
                    span: Span::new(Position::new(18, 1, 19), Position::new(68, 4, 2)),
                    offset: 0,
                    options: PluralOrSelectOptions(vec![
                        (
                            "one",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "item".to_string(),
                                    span: Span::new(
                                        Position::new(45, 2, 8),
                                        Position::new(49, 2, 12)
                                    ),
                                }],
                                span: Span::new(Position::new(44, 2, 7), Position::new(50, 2, 13)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "items".to_string(),
                                    span: Span::new(
                                        Position::new(60, 3, 10),
                                        Position::new(65, 3, 15)
                                    ),
                                },],
                                span: Span::new(Position::new(59, 3, 9), Position::new(66, 3, 16)),
                            },
                        ),
                    ]),
                },
            ])
        )
    }

    #[test]
    fn plural_arg_2() {
        assert_eq!(
            Parser::new(indoc! {"\
            You have {itemCount, plural,
              =0 {no items}
              one {1 item}
              other {{itemCount} items}
            }."})
            .parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "You have ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(9, 1, 10)),
                },
                AstElement::Plural {
                    value: "itemCount",
                    plural_type: PluralType::Cardinal,
                    span: Span::new(Position::new(9, 1, 10), Position::new(89, 5, 2)),
                    offset: 0,
                    options: PluralOrSelectOptions(vec![
                        (
                            "=0",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "no items".to_string(),
                                    span: Span::new(
                                        Position::new(35, 2, 7),
                                        Position::new(43, 2, 15)
                                    ),
                                },],
                                span: Span::new(Position::new(34, 2, 6), Position::new(44, 2, 16)),
                            },
                        ),
                        (
                            "one",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "1 item".to_string(),
                                    span: Span::new(
                                        Position::new(52, 3, 8),
                                        Position::new(58, 3, 14)
                                    ),
                                },],
                                span: Span::new(Position::new(51, 3, 7), Position::new(59, 3, 15)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Argument {
                                        value: "itemCount",
                                        span: Span::new(
                                            Position::new(69, 4, 10),
                                            Position::new(80, 4, 21)
                                        ),
                                    },
                                    AstElement::Literal {
                                        value: " items".to_string(),
                                        span: Span::new(
                                            Position::new(80, 4, 21),
                                            Position::new(86, 4, 27)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(68, 4, 9), Position::new(87, 4, 28)),
                            },
                        ),
                    ]),
                },
                AstElement::Literal {
                    value: ".".to_string(),
                    span: Span::new(Position::new(89, 5, 2), Position::new(90, 5, 3)),
                },
            ])
        )
    }

    #[test]
    fn plural_arg_with_offset_1() {
        assert_eq!(
            Parser::new(indoc! {"\
            You have {itemCount, plural, offset: 2
              =0 {no items}
              one {1 item}
              other {{itemCount} items}
            }."})
            .parse(),
            Ok(vec![
                AstElement::Literal {
                    value: "You have ".to_string(),
                    span: Span::new(Position::new(0, 1, 1), Position::new(9, 1, 10)),
                },
                AstElement::Plural {
                    value: "itemCount",
                    plural_type: PluralType::Cardinal,
                    span: Span::new(Position::new(9, 1, 10), Position::new(99, 5, 2)),
                    offset: 2,
                    options: PluralOrSelectOptions(vec![
                        (
                            "=0",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "no items".to_string(),
                                    span: Span::new(
                                        Position::new(45, 2, 7),
                                        Position::new(53, 2, 15)
                                    ),
                                },],
                                span: Span::new(Position::new(44, 2, 6), Position::new(54, 2, 16)),
                            },
                        ),
                        (
                            "one",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "1 item".to_string(),
                                    span: Span::new(
                                        Position::new(62, 3, 8),
                                        Position::new(68, 3, 14)
                                    ),
                                },],
                                span: Span::new(Position::new(61, 3, 7), Position::new(69, 3, 15)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Argument {
                                        value: "itemCount",
                                        span: Span::new(
                                            Position::new(79, 4, 10),
                                            Position::new(90, 4, 21)
                                        ),
                                    },
                                    AstElement::Literal {
                                        value: " items".to_string(),
                                        span: Span::new(
                                            Position::new(90, 4, 21),
                                            Position::new(96, 4, 27)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(78, 4, 9), Position::new(97, 4, 28)),
                            },
                        ),
                    ]),
                },
                AstElement::Literal {
                    value: ".".to_string(),
                    span: Span::new(Position::new(99, 5, 2), Position::new(100, 5, 3)),
                },
            ])
        )
    }

    #[test]
    fn plural_arg_with_escaped_nested_message() {
        assert_eq!(
            Parser::new(indoc! {"\
            {itemCount, plural,
              one {item'}'}
              other {items'}'}
            }"})
            .parse(),
            Ok(vec![AstElement::Plural {
                value: "itemCount",
                plural_type: PluralType::Cardinal,
                span: Span::new(Position::new(0, 1, 1), Position::new(56, 4, 2)),
                offset: 0,
                options: PluralOrSelectOptions(vec![
                    (
                        "one",
                        PluralOrSelectOption {
                            value: vec![AstElement::Literal {
                                value: "item}".to_string(),
                                span: Span::new(Position::new(27, 2, 8), Position::new(34, 2, 15)),
                            },],
                            span: Span::new(Position::new(26, 2, 7), Position::new(35, 2, 16)),
                        },
                    ),
                    (
                        "other",
                        PluralOrSelectOption {
                            value: vec![AstElement::Literal {
                                value: "items}".to_string(),
                                span: Span::new(Position::new(45, 3, 10), Position::new(53, 3, 18)),
                            },],
                            span: Span::new(Position::new(44, 3, 9), Position::new(54, 3, 19)),
                        },
                    ),
                ]),
            }])
        )
    }

    #[test]
    fn select_arg_1() {
        assert_eq!(
            Parser::new(indoc! {"\
              {gender, select,
                  male {He}
                  female {She}
                  other {They}
              } will respond shortly.
            "})
            .parse(),
            Ok(vec![
                AstElement::Select {
                    value: "gender",
                    span: Span::new(Position::new(0, 1, 1), Position::new(66, 5, 2)),
                    options: PluralOrSelectOptions(vec![
                        (
                            "male",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "He".to_string(),
                                    span: Span::new(
                                        Position::new(27, 2, 11),
                                        Position::new(29, 2, 13)
                                    ),
                                },],
                                span: Span::new(Position::new(26, 2, 10), Position::new(30, 2, 14)),
                            },
                        ),
                        (
                            "female",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "She".to_string(),
                                    span: Span::new(
                                        Position::new(43, 3, 13),
                                        Position::new(46, 3, 16)
                                    ),
                                },],
                                span: Span::new(Position::new(42, 3, 12), Position::new(47, 3, 17)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "They".to_string(),
                                    span: Span::new(
                                        Position::new(59, 4, 12),
                                        Position::new(63, 4, 16)
                                    ),
                                },],
                                span: Span::new(Position::new(58, 4, 11), Position::new(64, 4, 17)),
                            },
                        ),
                    ]),
                },
                AstElement::Literal {
                    value: " will respond shortly.\n".to_string(),
                    span: Span::new(Position::new(66, 5, 2), Position::new(89, 6, 1)),
                },
            ])
        )
    }

    #[test]
    fn select_arg_with_nested_arguments() {
        assert_eq!(
            Parser::new(indoc! {"\
              {taxableArea, select,
                  yes {An additional {taxRate, number, percent} tax will be collected.}
                  other {No taxes apply.}
              }
            "})
            .parse(),
            Ok(vec![
                AstElement::Select {
                    value: "taxableArea",
                    span: Span::new(Position::new(0, 1, 1), Position::new(125, 4, 2)),
                    options: PluralOrSelectOptions(vec![
                        (
                            "yes",
                            PluralOrSelectOption {
                                value: vec![
                                    AstElement::Literal {
                                        value: "An additional ".to_string(),
                                        span: Span::new(
                                            Position::new(31, 2, 10),
                                            Position::new(45, 2, 24)
                                        ),
                                    },
                                    AstElement::Number {
                                        value: "taxRate",
                                        span: Span::new(
                                            Position::new(45, 2, 24),
                                            Position::new(71, 2, 50)
                                        ),
                                        style: Some(NumberArgStyle::Style("percent",),),
                                    },
                                    AstElement::Literal {
                                        value: " tax will be collected.".to_string(),
                                        span: Span::new(
                                            Position::new(71, 2, 50),
                                            Position::new(94, 2, 73)
                                        ),
                                    },
                                ],
                                span: Span::new(Position::new(30, 2, 9), Position::new(95, 2, 74)),
                            },
                        ),
                        (
                            "other",
                            PluralOrSelectOption {
                                value: vec![AstElement::Literal {
                                    value: "No taxes apply.".to_string(),
                                    span: Span::new(
                                        Position::new(107, 3, 12),
                                        Position::new(122, 3, 27)
                                    ),
                                },],
                                span: Span::new(
                                    Position::new(106, 3, 11),
                                    Position::new(123, 3, 28)
                                ),
                            },
                        ),
                    ]),
                },
                AstElement::Literal {
                    value: "\n".to_string(),
                    span: Span::new(Position::new(125, 4, 2), Position::new(126, 5, 1)),
                },
            ])
        )
    }

    // TODO: port https://github.com/formatjs/formatjs/blob/main/packages/intl-messageformat-parser/tests/nested.test.ts
    // TODO: port https://github.com/formatjs/formatjs/blob/main/packages/intl-messageformat-parser/tests/index.test.ts
}
