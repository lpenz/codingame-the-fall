// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error;
use std::fmt;
use std::io;
use std::num;

use crate::sqrid;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    Sqrid(sqrid::Error),
    CellParseError,
    LineIteratorEnded,
    InvalidInput,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}
impl From<sqrid::Error> for Error {
    fn from(e: sqrid::Error) -> Error {
        Error::Sqrid(e)
    }
}
