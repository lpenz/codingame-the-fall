// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::str::FromStr;

use super::core::*;
use super::error::*;

const DEBUG: bool = false;

impl FromStr for Cell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let firstchar = s.chars().next().unwrap();
        let s = if firstchar == '-' { &s[1..] } else { s };
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

impl FromStr for Entity {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<_>>();
        Ok(Entity {
            qa: (v[0].parse::<u16>()?, v[1].parse::<u16>()?).try_into()?,
            qr: match v[2] {
                "TOP" => Qr::S,
                "LEFT" => Qr::E,
                "RIGHT" => Qr::W,
                _ => {
                    return Err(Error::InvalidInput);
                }
            },
        })
    }
}

fn lineread(lineit: &mut impl Iterator<Item = io::Result<String>>) -> Result<String, Error> {
    let line = lineit.next().ok_or(Error::LineIteratorEnded)??;
    if DEBUG {
        eprintln!("# {}", line);
    }
    Ok(line)
}

pub fn input_first(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    params: &mut Params,
    node: &mut Node,
) -> Result<(), Error> {
    let line = lineread(lineit)?;
    let wh = line.split(' ').collect::<Vec<_>>();
    params.width = wh[0].parse()?;
    params.height = wh[1].parse()?;
    for (y, lineres) in (0..params.height).zip(lineit.take(params.height as usize)) {
        let line = lineres?;
        if DEBUG {
            eprintln!("# {}", line);
        }
        let gridline = params.grid0.line_mut(y);
        for (x, cellnum) in line.split(' ').enumerate() {
            let firstchar = cellnum.chars().next().unwrap();
            if firstchar == '-' {
                let qa = Qa::try_from((x as u16, y))?;
                params.frozen.set_t(&qa);
            }
            gridline[x] = cellnum.parse()?;
        }
    }
    node.grid = params.grid0;
    let line = lineread(lineit)?;
    params.exit = Qa::try_from((line.parse()?, params.height - 1))?;
    Ok(())
}

pub fn input_ep1(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    _params: &Params,
    node: &mut Node,
) -> Result<(), Error> {
    let line = lineread(lineit)?;
    node.indy = line.parse()?;
    Ok(())
}

pub fn input_ep2(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    params: &Params,
    node: &mut Node,
) -> Result<(), Error> {
    input_ep1(lineit, params, node)?;
    let line = lineread(lineit)?;
    let num_rocks = line.parse::<usize>()?;
    for irock in IRock::iter() {
        if usize::from(irock) < num_rocks {
            let rock_str = lineread(lineit)?;
            node.rock[irock] = Some(rock_str.parse()?);
        } else {
            node.rock[irock] = None;
        }
    }
    Ok(())
}
