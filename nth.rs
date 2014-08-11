/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ascii::StrAsciiExt;

use ast::*;


/// Parse the An+B notation, as found in the ``:nth-child()`` selector.
/// The input is typically the arguments of a function component value.
/// Return Some((A, B)), or None for a syntax error.
pub fn parse_nth(input: &[ComponentValue]) -> Result<(i32, i32), ()> {
    let iter = &mut input.skip_whitespace();
    match iter.next() {
        Some(&Number(ref value)) => match value.int_value {
            Some(b) => parse_end(iter, 0, b as i32),
            _ => Err(()),
        },
        Some(&Dimension(ref value, ref unit)) => match value.int_value {
            Some(a) => {
                let unit = unit.as_slice().to_ascii_lower();
                let unit = unit.as_slice();
                match unit {
                    "n" => parse_b(iter, a as i32),
                    "n-" => parse_signless_b(iter, a as i32, -1),
                    _ => match parse_n_dash_digits(unit) {
                        Some(b) => parse_end(iter, a as i32, b),
                        _ => Err(())
                    },
                }
            },
            _ => Err(()),
        },
        Some(&Ident(ref value)) => {
            let ident = value.as_slice().to_ascii_lower();
            let ident = ident.as_slice();
            match ident {
                "even" => parse_end(iter, 2, 0),
                "odd" => parse_end(iter, 2, 1),
                "n" => parse_b(iter, 1),
                "-n" => parse_b(iter, -1),
                "n-" => parse_signless_b(iter, 1, -1),
                "-n-" => parse_signless_b(iter, -1, -1),
                _ if ident.starts_with("-") => match parse_n_dash_digits(ident.slice_from(1)) {
                    Some(b) => parse_end(iter, -1, b),
                    _ => Err(())
                },
                _ =>  match parse_n_dash_digits(ident) {
                    Some(b) => parse_end(iter, 1, b),
                    _ => Err(())
                },
            }
        },
        Some(&Delim('+')) => match iter.iter_with_whitespace.next() {
            Some(&Ident(ref value)) => {
                let ident = value.as_slice().to_ascii_lower();
                let ident = ident.as_slice();
                match ident {
                    "n" => parse_b(iter, 1),
                    "n-" => parse_signless_b(iter, 1, -1),
                    _ => match parse_n_dash_digits(ident) {
                        Some(b) => parse_end(iter, 1, b),
                        _ => Err(())
                    },
                }
            },
            _ => Err(())
        },
        _ => Err(())
    }
}


type Nth = Result<(i32, i32), ()>;
type Iter<'a> = SkipWhitespaceIterator<'a>;

fn parse_b(iter: &mut Iter, a: i32) -> Nth {
    match iter.next() {
        None => Ok((a, 0)),
        Some(&Delim('+')) => parse_signless_b(iter, a, 1),
        Some(&Delim('-')) => parse_signless_b(iter, a, -1),
        Some(&Number(ref value)) => match value.int_value {
            Some(b) if has_sign(value) => parse_end(iter, a, b as i32),
            _ => Err(()),
        },
        _ => Err(())
    }
}

fn parse_signless_b(iter: &mut Iter, a: i32, b_sign: i32) -> Nth {
    match iter.next() {
        Some(&Number(ref value)) => match value.int_value {
            Some(b) if !has_sign(value) => parse_end(iter, a, b_sign * (b as i32)),
            _ => Err(()),
        },
        _ => Err(())
    }
}

fn parse_end(iter: &mut Iter, a: i32, b: i32) -> Nth {
    match iter.next() {
        None => Ok((a, b)),
        Some(_) => Err(()),
    }
}

fn parse_n_dash_digits(string: &str) -> Option<i32> {
    if string.len() >= 3
    && string.starts_with("n-")
    && string.slice_from(2).chars().all(|c| match c { '0'..'9' => true, _ => false })
    {
        let result = from_str(string.slice_from(1));  // Include the minus sign
        assert!(result.is_some());
        result
    }
    else { None }
}

#[inline]
fn has_sign(value: &NumericValue) -> bool {
    match value.representation.as_bytes()[0] as char {
        '+' | '-' => true,
        _ => false
    }
}
