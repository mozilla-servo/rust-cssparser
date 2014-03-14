/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::vec;


#[deriving(Eq)]
pub struct NumericValue {
    representation: ~str,
    value: f64,
    int_value: Option<i64>,
}


#[deriving(Eq)]
pub struct SourceLocation {
    line: uint,  // First line is 1
    column: uint,  // First character of a line is at column 1
}


pub type Node = (ComponentValue, SourceLocation);  // TODO this is not a good name


#[deriving(Eq)]
pub enum ComponentValue {
    // Preserved tokens.
    Ident(~str),
    AtKeyword(~str),
    Hash(~str),
    IDHash(~str),  // Hash that is a valid ID selector.
    String(~str),
    URL(~str),
    Delim(char),
    Number(NumericValue),
    Percentage(NumericValue),
    Dimension(NumericValue, ~str),
    UnicodeRange(u32, u32),  // (start, end) of range
    WhiteSpace,
    Colon,  // :
    Semicolon,  // ;
    Comma,  // ,
    IncludeMatch, // ~=
    DashMatch, // |=
    PrefixMatch, // ^=
    SuffixMatch, // $=
    SubstringMatch, // *=
    Column, // ||
    CDO,  // <!--
    CDC,  // -->

    // Function
    Function(~str, ~[ComponentValue]),  // name, arguments

    // Simple block
    ParenthesisBlock(~[ComponentValue]),  // (…)
    SquareBracketBlock(~[ComponentValue]),  // […]
    CurlyBracketBlock(~[Node]),  // {…}

    // These are always invalid
    BadURL,
    BadString,
    CloseParenthesis, // )
    CloseSquareBracket, // ]
    CloseCurlyBracket, // }
}


#[deriving(Eq)]
pub struct Declaration {
    location: SourceLocation,
    name: ~str,
    value: ~[ComponentValue],
    important: bool,
}

#[deriving(Eq)]
pub struct QualifiedRule {
    location: SourceLocation,
    prelude: ~[ComponentValue],
    block: ~[Node],
}

#[deriving(Eq)]
pub struct AtRule {
    location: SourceLocation,
    name: ~str,
    prelude: ~[ComponentValue],
    block: Option<~[Node]>,
}

#[deriving(Eq)]
pub enum DeclarationListItem {
    Declaration(Declaration),
    // A better idea for a name that means "at-rule" but is not "AtRule"?
    DeclAtRule(AtRule),
}

#[deriving(Eq)]
pub enum Rule {
    QualifiedRule(QualifiedRule),
    AtRule(AtRule),
}

#[deriving(Eq)]
pub struct SyntaxError {
    location: SourceLocation,
    reason: ErrorReason,
}

#[deriving(Eq)]
pub enum ErrorReason {
    ErrEmptyInput,  // Parsing a single "thing", found only whitespace.
    ErrExtraInput,  // Found more non-whitespace after parsing a single "thing".
    ErrMissingQualifiedRuleBlock,  // EOF in a qualified rule prelude, before '{'
    ErrInvalidDeclarationSyntax,
    ErrInvalidBangImportantSyntax,
    // This is meant to be extended
}

impl fmt::Show for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{:u}:{:u} {:?}", self.location.line, self.location.column, self.reason)
    }
}


pub trait SkipWhitespaceIterable<'a> {
    fn skip_whitespace(self) -> SkipWhitespaceIterator<'a>;
}

impl<'a> SkipWhitespaceIterable<'a> for &'a [ComponentValue] {
    fn skip_whitespace(self) -> SkipWhitespaceIterator<'a> {
        SkipWhitespaceIterator{ iter_with_whitespace: self.iter() }
    }
}

#[deriving(Clone)]
pub struct SkipWhitespaceIterator<'a> {
    iter_with_whitespace: vec::Items<'a, ComponentValue>,
}

impl<'a> Iterator<&'a ComponentValue> for SkipWhitespaceIterator<'a> {
    fn next(&mut self) -> Option<&'a ComponentValue> {
        for component_value in self.iter_with_whitespace {
            if component_value != &WhiteSpace { return Some(component_value) }
        }
        None
    }
}


pub trait MoveSkipWhitespaceIterable {
    fn move_skip_whitespace(self) -> MoveSkipWhitespaceIterator;
}

impl MoveSkipWhitespaceIterable for ~[ComponentValue] {
    fn move_skip_whitespace(self) -> MoveSkipWhitespaceIterator {
        MoveSkipWhitespaceIterator{ iter_with_whitespace: self.move_iter() }
    }
}

pub struct MoveSkipWhitespaceIterator {
    iter_with_whitespace: vec::MoveItems<ComponentValue>,
}

impl Iterator<ComponentValue> for MoveSkipWhitespaceIterator {
    fn next(&mut self) -> Option<ComponentValue> {
        for component_value in self.iter_with_whitespace {
            if component_value != WhiteSpace { return Some(component_value) }
        }
        None
    }
}
