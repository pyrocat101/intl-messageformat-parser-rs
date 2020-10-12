use super::ast::{self, Ast, AstElement, Position, Span};
use std::cell::Cell;
use std::cmp;
use std::result;

type Result<T> = result::Result<T, ast::Error>;

#[derive(Clone, Debug)]
pub struct Parser<'s> {
  position: Cell<Position>,
  message: &'s str,
  // TODO: parser context and parser options
}

impl<'s> Parser<'s> {
  pub fn new(message: &'s str) -> Parser<'s> {
    Parser {
      message,
      position: Cell::new(Position { offset: 0, line: 1, column: 1 }),
    }
  }

  pub fn parse(&mut self) -> Result<Ast> {
    assert_eq!(self.offset(), 0, "parser can only be used once");
    let mut elements: Vec<AstElement> = vec![];

    loop {
      elements.push(match self.peek() {
        None => break,
        Some('{') => self.parse_argument()?,
        Some(_) => self.parse_literal()?,
      })
    }

    return Ok(elements);
  }

  fn position(&self) -> Position {
    return self.position.get();
  }

  fn parse_literal(&self) -> Result<AstElement> {
    let start = self.position();
    self.bump_until('{');

    let span = Span::new(start, self.position());
    return Ok(AstElement::Literal { span, value: self.slice(&span) });
  }

  fn slice(&self, span: &Span) -> &str {
    return &self.message[span.start.offset..span.end.offset];
  }

  fn parse_argument(&self) -> Result<AstElement> {
    let opening_brace_position = self.position();
    self.bump();

    let argument_start_offset = self.offset();
    if self.bump_until('}') {
      let argument_end_offset = self.offset();
      self.bump();
      let closing_brace_position = self.position();

      return Ok(AstElement::Argument {
        // value does not include the opening and closing braces.
        value: &self.message[argument_start_offset..argument_end_offset],
        span: Span::new(opening_brace_position, closing_brace_position),
      });
    } else {
      // Unclosed argument
      return Err(self.error(
        ast::ErrorKind::UnclosedArgumentBrace,
        Span::new(opening_brace_position, self.position()),
      ));
    }
  }

  fn error(&self, kind: ast::ErrorKind, span: Span) -> ast::Error {
    return ast::Error { kind, message: self.message.to_string(), span };
  }

  fn offset(&self) -> usize {
    return self.position().offset;
  }

  /// Return the character at the current position of the parser.
  ///
  /// This panics if the current position does not point to a valid char.
  fn char(&self) -> char {
    return self.char_at(self.offset());
  }

  /// Return the character at the given position.
  ///
  /// This panics if the given position does not point to a valid char.
  fn char_at(&self, i: usize) -> char {
    return self.message[i..]
      .chars()
      .next()
      .unwrap_or_else(|| panic!("expected char at offset {}", i));
  }

  /// Bump the parser to the next Unicode scalar value.
  ///
  /// If the end of the input has been reached after bump, then `false` is returned.
  fn bump(&self) -> bool {
    if self.is_eof() {
      return false;
    }
    let Position { mut offset, mut line, mut column } = self.position();
    if self.char() == '\n' {
      line = line.checked_add(1).unwrap();
      column = 1;
    } else {
      column = column.checked_add(1).unwrap();
    }
    offset += self.char().len_utf8();
    self.position.set(Position { offset, line, column });
    return self.message[self.offset()..].chars().next().is_some();
  }

  /// Bump the parser to the target offset.
  ///
  /// If target offset is beyond the end of the input, bump the parser to the end of the input.
  fn bump_to(&self, target_offset: usize) -> () {
    assert!(
      self.offset() < target_offset,
      "target_offset {} must be greater than the current offset {})",
      target_offset,
      self.offset()
    );

    let target_offset = cmp::min(target_offset, self.message.len());
    loop {
      let offset = self.offset();

      if self.offset() == target_offset {
        break;
      }
      assert!(
        offset < target_offset,
        "target_offset is at invalid unicode byte boundary: {}",
        target_offset
      );

      let has_more = self.bump();
      if !has_more {
        break;
      }
    }
  }

  // /// If the substring starting at the current position of the parser has
  // /// the given prefix, then bump the parser to the character immediately
  // /// following the prefix and return true. Otherwise, don't bump the parser
  // /// and return false.
  // fn bump_if(&mut self, prefix: &str) -> bool {
  //   if self.message[self.offset()..].starts_with(prefix) {
  //     for _ in 0..prefix.chars().count() {
  //       self.bump();
  //     }
  //     true
  //   } else {
  //     false
  //   }
  // }

  /// Bump the parser until the pattern character is found and return `true`.
  /// Otherwise bump to the end of the file and return `false`.
  fn bump_until<'a>(&'a self, pattern: char) -> bool {
    let current_offset = self.offset();
    if let Some(delta) = self.message[current_offset..].find(pattern) {
      self.bump_to(current_offset + delta);
      return true;
    } else {
      self.bump_to(self.message.len());
      return false;
    }
  }

  /// Peek at the next character in the input without advancing the parser.
  ///
  /// If the input has been exhausted, then this returns `None`.
  fn peek(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }
    return self.message[self.offset()..].chars().next();
  }

  /// Returns true if the next call to `bump` would return false.
  fn is_eof(&self) -> bool {
    self.offset() == self.message.len()
  }
}
