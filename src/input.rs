// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::str::FromStr;

use crate::core::*;
use crate::error::*;

impl FromStr for Cell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Cell::Type0),
            "1" => Ok(Cell::Type1),
            "2" => Ok(Cell::Type2),
            "3" => Ok(Cell::Type3),
            "4" => Ok(Cell::Type4),
            "5" => Ok(Cell::Type5),
            "6" => Ok(Cell::Type6),
            "7" => Ok(Cell::Type7),
            "8" => Ok(Cell::Type8),
            "9" => Ok(Cell::Type9),
            "10" => Ok(Cell::Type10),
            "11" => Ok(Cell::Type11),
            "12" => Ok(Cell::Type12),
            "13" => Ok(Cell::Type13),
            _ => Err(Error::CellParseError),
        }
    }
}

fn lineread(lineit: &mut impl Iterator<Item = io::Result<String>>) -> Result<String, Error> {
    Ok(lineit.next().ok_or(Error::LineIteratorEnded)??)
}

pub fn input_first(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    params: &mut Params,
    _node: &mut Node,
) -> Result<(), Error> {
    let line = lineread(lineit)?;
    let wh = line.split(' ').collect::<Vec<_>>();
    params.width = wh[0].parse()?;
    params.height = wh[1].parse()?;
    for (y, lineres) in (0..params.height).zip(lineit.take(params.height as usize)) {
        let line = lineres?;
        let gridline = params.grid.line_mut(y);
        for (x, cellnum) in line.split(' ').enumerate() {
            gridline[x] = cellnum.parse()?;
        }
    }
    let x = lineread(lineit)?;
    params.exit = Qa::try_from((x.parse()?, params.height - 1))?;
    Ok(())
}

pub fn input(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    _params: &Params,
    node: &mut Node,
) -> Result<(), Error> {
    let line = lineread(lineit)?;
    let indy_str = line.split(' ').collect::<Vec<_>>();
    node.indy = (indy_str[0].parse::<u16>()?, indy_str[1].parse::<u16>()?).try_into()?;
    node.dir = match indy_str[2] {
        "TOP" => Qr::S,
        "LEFT" => Qr::E,
        "RIGHT" => Qr::W,
        _ => {
            return Err(Error::InvalidInput);
        }
    };
    Ok(())
}
