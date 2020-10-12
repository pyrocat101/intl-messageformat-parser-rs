use super::js_intl::*;
use std::fmt;

/// The type of an error that occurred while building an AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
  UnclosedArgumentBrace,
  // TODO
}

/// A single position in an ICU message.
///
/// A position encodes one half of a span, and include the code unit offset, line
/// number and column number.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
  pub offset: usize,
  pub line: usize,
  pub column: usize,
}

impl Position {
  pub fn new(offset: usize, line: usize, column: usize) -> Position {
    return Position { offset, line, column };
  }
}

impl fmt::Debug for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Position(o: {:?}, l: {:?}, c: {:?})",
      self.offset, self.line, self.column
    )
  }
}

/// Span represents the position information of a single AST item.
///
/// All span positions are absolute byte offsets that can be used on the
/// original regular expression that was parsed.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
  /// The start byte offset.
  pub start: Position,
  /// The end byte offset.
  pub end: Position,
}

impl fmt::Debug for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Span({:?}, {:?})", self.start, self.end)
  }
}

impl Span {
  /// Create a new span with the given positions.
  pub fn new(start: Position, end: Position) -> Span {
    Span { start: start, end: end }
  }
}

/// An error that occurred while parsing an ICU message into an abstract
/// syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
  /// The kind of error.
  pub kind: ErrorKind,
  /// The original message that the parser generated the error from. Every
  /// span in an error is a valid range into this string.
  pub message: String,
  /// The span of this error.
  pub span: Span,
}

/// An abstract syntax tree for a ICU message. Adapted from:
/// https://github.com/formatjs/formatjs/blob/c03d4989323a33765798acdd74fb4f5b01f0bdcd/packages/intl-messageformat-parser/src/types.ts
pub type Ast<'s> = Vec<AstElement<'s>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AstElement<'s> {
  /// Raw text
  Literal { value: &'s str, span: Span },
  /// Variable w/o any format, e.g `var` in `this is a {var}`
  Argument { value: &'s str, span: Span },
  /// Variable w/ number format
  Number { value: &'s str, span: Span, style: NumberSkeleton<'s> },
  /// Variable w/ date format
  Date { value: &'s str, span: Span, style: DateTimeSkeleton<'s> },
  /// Variable w/ time format
  Time { value: &'s str, span: Span, options: DateTimeSkeleton<'s> },
  /// Variable w/ select format
  Select {
    value: &'s str,
    span: Span,
    options: Vec<(&'s str, PluralOrSelectOption<'s>)>,
  },
  /// Variable w/ plural format
  Plural {
    value: &'s str,
    span: Span,
    options: Vec<(&'s str, PluralOrSelectOption<'s>)>,
  },
  /// XML-like tag
  Tag { value: &'s str, span: Span, children: Box<AstElement<'s>> },
  /// Only possible within plural argument.
  /// This is the `#` symbol that will be substituted with the count.
  Pound(Span),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberSkeleton<'s> {
  tokens: Vec<NumberSkeletonToken<'s>>,
  span: Span,
  parsed_options: Option<JsIntlNumberFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberSkeletonToken<'s> {
  stem: &'s str,
  options: Vec<&'s str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeSkeleton<'s> {
  pattern: &'s str,
  span: Span,
  parsed_options: Option<JsIntlDateTimeFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluralOrSelectOption<'s> {
  value: AstElement<'s>,
  span: Span,
}
