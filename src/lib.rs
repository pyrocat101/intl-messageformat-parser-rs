mod ast;
mod js_intl;
mod parser;

pub use ast::{Ast, AstElement, Position, Span};
pub use parser::Parser;

#[cfg(test)]
mod tests {
    use crate::ast::{AstElement, Position, Span};
    use crate::parser::Parser;

    #[test]
    fn trivial_1() {
        assert_eq!(
            Parser::new("a").parse(),
            Ok(vec![AstElement::Literal {
                value: "a",
                span: Span::new(Position::new(0, 1, 1), Position::new(1, 1, 2))
            }])
        );
    }

    #[test]
    fn trivial_2() {
        assert_eq!(
            Parser::new("中文").parse(),
            Ok(vec![AstElement::Literal {
                value: "中文",
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
                    value: "a ",
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
                    value: " \nc",
                    span: Span::new(
                        Position::new(5, 1, 6),
                        Position::new(8, 2, 2)
                    )
                },
            ])
        );
    }
}
