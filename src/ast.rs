use super::js_intl::*;
use std::fmt;

/// The type of an error that occurred while building an AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Argumernt is unclosed (e.g. `{0`)
    UnclosedArgumentBrace,
    /// Argument is empty (e.g. `{}`).
    EmptyArgument,
    /// Argument is malformed (e.g. `{foo!}``)
    MalformedArgument,
    /// Expect an argument type (e.g. `{foo,}`)
    ExpectArgumentType,
    /// Unsupported argument type (e.g. `{foo,foo}`)
    InvalidArgumentType,
    /// Expect an argument style (e.g. `{foo, number, }`)
    ExpectArgumentStyle,
    /// The number skeleton is invalid.
    InvalidNumberSkeleton,
    /// The date time skeleton is invalid.
    InvalidDateTimeSkeleton,
    /// Exepct a number skeleton following the `::` (e.g. `{foo, number, ::}`)
    ExpectNumberSkeleton,
    /// Exepct a date time skeleton following the `::` (e.g. `{foo, date, ::}`)
    ExpectDateTimeSkeleton,
    /// Unmatched apostrophes in the argument style (e.g. `{foo, number, 'test`)
    UnclosedQuoteInArgumentStyle,
    /// Missing select argument options (e.g. `{foo, select}`)
    ExpectSelectArgumentOptions,

    /// Expecting an offset value in `plural` or `selectordinal` argument (e.g `{foo, plural, offset}`)
    ExpectPluralArgumentOffsetValue,
    /// Offset value in `plural` or `selectordinal` is invalid (e.g. `{foo, plural, offset: x}`)
    InvalidPluralArgumentOffsetValue,

    /// Expecting a selector in `select` argument (e.g `{foo, select}`)
    ExpectSelectArgumentSelector,
    /// Expecting a selector in `plural` or `selectordinal` argument (e.g `{foo, plural}`)
    ExpectPluralArgumentSelector,

    /// Expecting a message fragment after the `select` selector (e.g. `{foo, select, apple}`)
    ExpectSelectArgumentSelectorFragment,
    /// Expecting a message fragment after the `plural` or `selectordinal` selector
    /// (e.g. `{foo, plural, one}`)
    ExpectPluralArgumentSelectorFragment,

    /// Selector in `plural` or `selectordinal` is malformed (e.g. `{foo, plural, =x {#}}`)
    InvalidPluralArgumentSelector,

    /// Duplicate selectors in `plural` or `selectordinal` argument.
    /// (e.g. {foo, plural, one {#} one {#}})
    DuplicatePluralArgumentSelector,
    /// Duplicate selectors in `select` argument.
    /// (e.g. {foo, select, apple {apple} apple {apple}})
    DuplicateSelectArgumentSelector,

    /// Plural or select argument option must have `other` clause.
    MissingOtherClause,
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
        Position { offset, line, column }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position(o: {:?}, l: {:?}, c: {:?})", self.offset, self.line, self.column)
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
        Span { start, end }
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
pub enum PluralType {
    Cardinal,
    Ordinal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AstElement<'s> {
    /// Raw text
    Literal { value: String, span: Span },
    /// Variable w/o any format, e.g `var` in `this is a {var}`
    Argument { value: &'s str, span: Span },
    /// Variable w/ number format
    Number { value: &'s str, span: Span, style: Option<NumberArgStyle<'s>> },
    /// Variable w/ date format
    Date { value: &'s str, span: Span, style: Option<DateTimeArgStyle<'s>> },
    /// Variable w/ time format
    Time { value: &'s str, span: Span, style: Option<DateTimeArgStyle<'s>> },
    /// Variable w/ select format
    Select { value: &'s str, span: Span, options: Vec<(&'s str, PluralOrSelectOption<'s>)> },
    /// Variable w/ plural format
    Plural {
        value: &'s str,
        plural_type: PluralType,
        span: Span,
        // TODO: want to use double here but it does not implement Eq trait.
        offset: i64,
        options: Vec<(&'s str, PluralOrSelectOption<'s>)>,
    },
    /// Only possible within plural argument.
    /// This is the `#` symbol that will be substituted with the count.
    Pound(Span),
    /// XML-like tag
    Tag { value: &'s str, span: Span, children: Box<AstElement<'s>> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumberArgStyle<'s> {
    Style(&'s str),
    Skeleton(NumberSkeleton<'s>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberSkeleton<'s> {
    pub tokens: Vec<NumberSkeletonToken<'s>>,
    pub span: Span,
    pub parsed_options: Option<JsIntlNumberFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberSkeletonToken<'s> {
    pub stem: &'s str,
    pub options: Vec<&'s str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateTimeArgStyle<'s> {
    Style(&'s str),
    Skeleton(DateTimeSkeleton<'s>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeSkeleton<'s> {
    pub pattern: &'s str,
    pub span: Span,
    pub parsed_options: Option<JsIntlDateTimeFormatOptions>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluralOrSelectOption<'s> {
    pub value: Ast<'s>,
    pub span: Span,
}
