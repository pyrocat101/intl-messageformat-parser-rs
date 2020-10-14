use crate::ast::{self, Ast, AstElement, ErrorKind, Position, Span};
use crate::pattern_syntax::is_pattern_syntax;
use std::cell::Cell;
// use std::cmp;
use std::result;

type Result<T> = result::Result<T, ast::Error>;

#[derive(Clone, Debug)]
pub struct Parser<'s> {
  position: Cell<Position>,
  message: &'s str,
  should_ignore_tag: bool,
  // TODO: parser context and parser options
}

impl<'s> Parser<'s> {
  pub fn new(message: &'s str) -> Parser<'s> {
    Parser {
      message,
      position: Cell::new(Position { offset: 0, line: 1, column: 1 }),
      // TODO: support configuring this.
      should_ignore_tag: true,
    }
  }

  pub fn parse(&mut self) -> Result<Ast> {
    assert_eq!(self.offset(), 0, "parser can only be used once");
    let mut elements: Vec<AstElement> = vec![];

    loop {
      if self.is_eof() {
        break;
      }
      elements.push(match self.char() {
        '{' => self.parse_argument()?,
        _ => self.parse_literal()?,
      })
    }

    Ok(elements)
  }

  fn position(&self) -> Position {
    self.position.get()
  }

  fn parse_literal(&self) -> Result<AstElement> {
    let start = self.position();

    let mut value = String::new();
    loop {
      if self.bump_if("''") {
        value.push('\'');
      } else if let Some(fragment) = self.try_parse_quote() {
        value.push_str(&fragment);
      } else if let Some(fragment) = self.try_parse_unquoted() {
        value.push(fragment);
      } else if let Some(fragment) = self.try_parse_left_angle_bracket() {
        value.push_str(&fragment);
      } else {
        // TODO: remove this after more rules are added.
        assert!(self.is_eof() || self.char() == '{');
        break;
      }
    }

    let span = Span::new(start, self.position());
    Ok(AstElement::Literal { span, value })
  }

  /// Starting with ICU 4.8, an ASCII apostrophe only starts quoted text if it immediately precedes
  /// a character that requires quoting (that is, "only where needed"), and works the same in
  /// nested messages as on the top level of the pattern. The new behavior is otherwise compatible.
  fn try_parse_quote(&self) -> Option<String> {
    if self.is_eof() || self.char() != '\'' {
      return None;
    }

    // Parse escaped char following the apostrophe, or early return if there is no escaped char.
    // TODO
    let is_in_plural_option = false;
    // Check if is valis escaped character
    match self.peek() {
      Some('{') | Some('<') | Some('>') | Some('}') => (),
      Some('#') if is_in_plural_option => (),
      _ => {
        return None;
      }
    }

    self.bump(); // apostrophe
    let mut value = self.char().to_string(); // escaped char
    self.bump();

    // read chars until the optional closing apostrophe is found
    loop {
      if self.is_eof() {
        break;
      }
      match self.char() {
        '\'' if self.peek() == Some('\'') => {
          value.push('\'');
          // Bump one more time because we need to skip 2 characters.
          self.bump();
        }
        '\'' => {
          // Optional closing apostrophe.
          self.bump();
          break;
        }
        c => value.push(c),
      }
      self.bump();
    }

    Some(value)
  }

  fn try_parse_unquoted(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }
    // TODO
    let is_in_plural_option = false;
    let is_in_nested_message_text = false;
    match self.char() {
      '<' | '{' => None,
      '#' if is_in_plural_option => None,
      '}' if is_in_nested_message_text => None,
      c => {
        self.bump();
        Some(c)
      }
    }
  }

  fn try_parse_left_angle_bracket(&self) -> Option<String> {
    // TODO
    let should_ignore_tag = false;

    if self.is_eof() || self.char() != '<' {
      return None;
    }

    if !should_ignore_tag {
      // make sure `<` is not parsed as regular opening angle bracket
      // NOTE: this requires infinite lookahead...
      // TODO
    }

    Some('<'.to_string())
  }

  fn parse_argument(&self) -> Result<AstElement> {
    let opening_brace_position = self.position();
    self.bump(); // `{`

    self.bump_space();

    if self.is_eof() {
      return Err(self.error(
        ErrorKind::UnclosedArgumentBrace,
        Span::new(opening_brace_position, self.position()),
      ));
    }

    if self.char() == '}' {
      self.bump();
      return Err(self.error(
        ErrorKind::EmptyArgument,
        Span::new(opening_brace_position, self.position()),
      ));
    }

    // argument name or number
    if self.is_eof()
      || self.char().is_whitespace()
      || is_pattern_syntax(self.char())
    {
      return Err(self.error(
        ErrorKind::MalformedArgument,
        Span::new(opening_brace_position, self.position()),
      ));
    }
    let value = self.parse_argument_name();

    self.bump_space();

    if self.is_eof() {
      return Err(self.error(
        ErrorKind::UnclosedArgumentBrace,
        Span::new(opening_brace_position, self.position()),
      ));
    }

    // TODO: support parsing comma here
    if self.char() != '}' {
      return Err(self.error(
        ErrorKind::MalformedArgument,
        Span::new(opening_brace_position, self.position()),
      ));
    }

    self.bump(); // `}`
    Ok(AstElement::Argument {
      // value does not include the opening and closing braces.
      value,
      span: Span::new(opening_brace_position, self.position()),
    })
  }

  fn parse_argument_name(&self) -> &str {
    let starting_position = self.position();

    self.bump();
    while !self.is_eof()
      && !self.char().is_whitespace()
      && !is_pattern_syntax(self.char())
    {
      self.bump();
    }

    let end_position = self.position();

    &self.message[starting_position.offset..end_position.offset]
  }

  fn error(&self, kind: ErrorKind, span: Span) -> ast::Error {
    ast::Error { kind, message: self.message.to_string(), span }
  }

  fn offset(&self) -> usize {
    self.position().offset
  }

  /// Return the character at the current position of the parser.
  ///
  /// This panics if the current position does not point to a valid char.
  fn char(&self) -> char {
    self.char_at(self.offset())
  }

  /// Return the character at the given position.
  ///
  /// This panics if the given position does not point to a valid char.
  fn char_at(&self, i: usize) -> char {
    self.message[i..]
      .chars()
      .next()
      .unwrap_or_else(|| panic!("expected char at offset {}", i))
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
    self.message[self.offset()..].chars().next().is_some()
  }

  // /// Bump the parser to the target offset.
  // ///
  // /// If target offset is beyond the end of the input, bump the parser to the end of the input.
  // fn bump_to(&self, target_offset: usize) {
  //   assert!(
  //     self.offset() < target_offset,
  //     "target_offset {} must be greater than the current offset {})",
  //     target_offset,
  //     self.offset()
  //   );

  //   let target_offset = cmp::min(target_offset, self.message.len());
  //   loop {
  //     let offset = self.offset();

  //     if self.offset() == target_offset {
  //       break;
  //     }
  //     assert!(
  //       offset < target_offset,
  //       "target_offset is at invalid unicode byte boundary: {}",
  //       target_offset
  //     );

  //     let has_more = self.bump();
  //     if !has_more {
  //       break;
  //     }
  //   }
  // }

  /// If the substring starting at the current position of the parser has
  /// the given prefix, then bump the parser to the character immediately
  /// following the prefix and return true. Otherwise, don't bump the parser
  /// and return false.
  fn bump_if(&self, prefix: &str) -> bool {
    if self.message[self.offset()..].starts_with(prefix) {
      for _ in 0..prefix.chars().count() {
        self.bump();
      }
      true
    } else {
      false
    }
  }

  // /// Bump the parser until the pattern character is found and return `true`.
  // /// Otherwise bump to the end of the file and return `false`.
  // fn bump_until(&self, pattern: char) -> bool {
  //   let current_offset = self.offset();
  //   if let Some(delta) = self.message[current_offset..].find(pattern) {
  //     self.bump_to(current_offset + delta);
  //     true
  //   } else {
  //     self.bump_to(self.message.len());
  //     false
  //   }
  // }

  /// advance the parser through all whitespace to the next non-whitespace byte.
  fn bump_space(&self) {
    while !self.is_eof() && self.char().is_whitespace() {
      self.bump();
    }
  }

  /// Peek at the *next* character in the input without advancing the parser.
  ///
  /// If the input has been exhausted, then this returns `None`.
  fn peek(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }
    self.message[self.offset() + self.char().len_utf8()..].chars().next()
  }

  /// Returns true if the next call to `bump` would return false.
  fn is_eof(&self) -> bool {
    self.offset() == self.message.len()
  }
}
