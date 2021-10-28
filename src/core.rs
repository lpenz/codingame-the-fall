// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

pub const MAX_WIDTH: u16 = 20;
pub const MAX_HEIGHT: u16 = 20;

pub type Sqrid = crate::sqrid_create!(MAX_WIDTH, MAX_HEIGHT, false);
pub type Qa = crate::qa_create!(Sqrid);
pub type Qr = crate::Qr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Type0,
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
    Type10,
    Type11,
    Type12,
    Type13,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Type0
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> char {
        match cell {
            Cell::Type0 => '0',
            Cell::Type1 => '1',
            Cell::Type2 => '2',
            Cell::Type3 => '3',
            Cell::Type4 => '4',
            Cell::Type5 => '5',
            Cell::Type6 => '6',
            Cell::Type7 => '7',
            Cell::Type8 => '8',
            Cell::Type9 => '9',
            Cell::Type10 => 'A',
            Cell::Type11 => 'B',
            Cell::Type12 => 'C',
            Cell::Type13 => 'D',
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Cell {
    pub fn enter(&self, dir: Qr) -> Option<Qr> {
        if dir == Qr::N {
            return None;
        }
        match self {
            Cell::Type0 => None,
            Cell::Type1 => Some(Qr::S),
            Cell::Type2 => match dir {
                Qr::E => Some(Qr::E),
                Qr::W => Some(Qr::W),
                _ => None,
            },
            Cell::Type3 => match dir {
                Qr::S => Some(Qr::S),
                _ => None,
            },
            Cell::Type4 => match dir {
                Qr::S => Some(Qr::W),
                Qr::W => Some(Qr::S),
                _ => None,
            },
            Cell::Type5 => match dir {
                Qr::S => Some(Qr::E),
                Qr::E => Some(Qr::S),
                _ => None,
            },
            Cell::Type6 => match dir {
                Qr::W => Some(Qr::W),
                Qr::E => Some(Qr::E),
                _ => None,
            },
            Cell::Type7 => match dir {
                Qr::S => Some(Qr::S),
                Qr::W => Some(Qr::S),
                _ => None,
            },
            Cell::Type8 => match dir {
                Qr::W => Some(Qr::S),
                Qr::E => Some(Qr::S),
                _ => None,
            },
            Cell::Type9 => match dir {
                Qr::S => Some(Qr::S),
                Qr::E => Some(Qr::S),
                _ => None,
            },
            Cell::Type10 => match dir {
                Qr::S => Some(Qr::W),
                _ => None,
            },
            Cell::Type11 => match dir {
                Qr::S => Some(Qr::E),
                _ => None,
            },
            Cell::Type12 => match dir {
                Qr::W => Some(Qr::S),
                _ => None,
            },
            Cell::Type13 => match dir {
                Qr::E => Some(Qr::S),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct Params {
    pub width: u16,
    pub height: u16,
    pub grid: crate::grid_create!(Sqrid, Cell),
    pub exit: Qa,
}

#[derive(Debug, Default)]
pub struct Node {
    pub indy: Qa,
    pub dir: Qr,
}
