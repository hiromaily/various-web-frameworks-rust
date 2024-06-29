//  ------------------------------------------------------------------
//  Airone
//  is a Rust library which provides a simple in-memory,
//  write-on-update database that is persisted
//  to an append-only transaction file.
//
//  Copyright © 2022,2023,2024 Massimo Gismondi
//
//  This file is part of Airone.
//  Airone is free software: you can redistribute it and/or
//  modify it under the terms of the GNU Affero General Public License
//  as published by the Free Software Foundation, either version 3
//  of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.

//  You should have received a copy of the GNU Affero General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.
//  ------------------------------------------------------------------

use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(ParseError),
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<ParseBoolError> for Error {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseError(ParseError {
            expectation: "bool parsing from string".to_string(),
            found: value.to_string(),
        })
    }
}
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(ParseError {
            expectation: "int parsing from string".to_string(),
            found: value.to_string(),
        })
    }
}
impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseError(ParseError {
            expectation: "float parsing from string".to_string(),
            found: value.to_string(),
        })
    }
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(io_error) => writeln!(f, "io-error: {}", io_error),
            Self::ParseError(e) => writeln!(
                f,
                "parse error. Expected: \"{}\", found:\"{}\"",
                e.expectation, e.found
            ),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    expectation: String,
    found: String,
}
impl ParseError {
    pub fn new(expectation: String, found: String) -> Self {
        Self { expectation, found }
    }
}
