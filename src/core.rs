// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::fmt;

pub const MAX_WIDTH: u16 = 20;
pub const MAX_HEIGHT: u16 = 20;

pub type Sqrid = crate::sqrid_create!(MAX_WIDTH, MAX_HEIGHT, false);
pub type Qa = crate::qa_create!(Sqrid);
pub type Qr = crate::Qr;
pub type Gridbool = crate::gridbool_create!(Sqrid);

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
    pub fn enter(&self, dir: &Qr) -> Option<Qr> {
        let dir = *dir;
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

    pub fn rot_cw(&self) -> Cell {
        match self {
            Cell::Type0 => Cell::Type0,
            Cell::Type1 => Cell::Type1,
            Cell::Type2 => Cell::Type3,
            Cell::Type3 => Cell::Type2,
            Cell::Type4 => Cell::Type5,
            Cell::Type5 => Cell::Type4,
            Cell::Type6 => Cell::Type7,
            Cell::Type7 => Cell::Type8,
            Cell::Type8 => Cell::Type9,
            Cell::Type9 => Cell::Type6,
            Cell::Type10 => Cell::Type11,
            Cell::Type11 => Cell::Type12,
            Cell::Type12 => Cell::Type13,
            Cell::Type13 => Cell::Type10,
        }
    }

    pub fn rot_cc(&self) -> Cell {
        match self {
            Cell::Type0 => Cell::Type0,
            Cell::Type1 => Cell::Type1,
            Cell::Type2 => Cell::Type3,
            Cell::Type3 => Cell::Type2,
            Cell::Type4 => Cell::Type5,
            Cell::Type5 => Cell::Type4,
            Cell::Type6 => Cell::Type9,
            Cell::Type7 => Cell::Type6,
            Cell::Type8 => Cell::Type7,
            Cell::Type9 => Cell::Type8,
            Cell::Type10 => Cell::Type13,
            Cell::Type11 => Cell::Type10,
            Cell::Type12 => Cell::Type11,
            Cell::Type13 => Cell::Type12,
        }
    }

    pub fn rotate(&self, rot: &Rotation) -> Cell {
        match rot {
            Rotation::Left => self.rot_cc(),
            Rotation::Right => self.rot_cw(),
            Rotation::Flip => self.rot_cw().rot_cw(),
        }
    }

    pub fn num_rot(&self) -> usize {
        match self {
            Cell::Type0 => 0,
            Cell::Type1 => 0,
            Cell::Type2 => 1,
            Cell::Type3 => 1,
            Cell::Type4 => 1,
            Cell::Type5 => 1,
            Cell::Type6 => 3,
            Cell::Type7 => 3,
            Cell::Type8 => 3,
            Cell::Type9 => 3,
            Cell::Type10 => 3,
            Cell::Type11 => 3,
            Cell::Type12 => 3,
            Cell::Type13 => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    Left,
    Right,
    Flip,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rotation::Left => write!(f, "LEFT"),
            Rotation::Right => write!(f, "RIGHT"),
            Rotation::Flip => write!(f, "RIGHT"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Entity {
    pub qa: Qa,
    pub qr: Qr,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.qa.tuple();
        write!(f, "({} {}, {})", t.0, t.1, self.qr)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Wait,
    Rotate { qa: Qa, rot: Rotation },
}

impl Action {
    pub fn new(qa: Qa, rotopt: Option<Rotation>) -> Action {
        if let Some(rot) = rotopt {
            Action::Rotate { qa, rot }
        } else {
            Action::Wait
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Wait => write!(f, "WAIT"),
            Action::Rotate { qa, rot } => {
                let t = qa.tuple();
                write!(f, "{} {} {}", t.0, t.1, rot)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Params {
    pub width: u16,
    pub height: u16,
    pub exit: Qa,
    pub frozen: Gridbool,
    pub grid0: crate::grid_create!(Sqrid, Cell),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Node {
    pub grid: crate::grid_create!(Sqrid, Cell),
    pub indy: Entity,
    pub num_rocks: usize,
    pub rock: [Option<Entity>; 10],
}

impl Node {
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Wait => {}
            Action::Rotate { qa, rot } => {
                self.grid[qa] = self.grid[qa].rotate(&rot);
            }
        }
    }
    pub fn apply_real(&mut self, action: Action) {
        match action {
            Action::Wait => {}
            Action::Rotate { qa, rot } => {
                if rot == Rotation::Flip {
                    self.grid[qa] = self.grid[qa].rotate(&Rotation::Right);
                } else {
                    self.grid[qa] = self.grid[qa].rotate(&rot);
                }
            }
        }
    }
    pub fn entity_step(&self, entity: &Entity) -> Option<Entity> {
        let cell = self.grid[entity.qa];
        if let Some(qr) = cell.enter(&entity.qr) {
            if let Some(qa) = entity.qa + qr {
                return Some(Entity { qa, qr });
            }
        }
        None
    }
    pub fn eval_indy_step(&mut self) -> bool {
        if let Some(indy) = self.entity_step(&self.indy) {
            self.indy = indy;
            return true;
        }
        false
    }
}

pub const ROT0: [Option<Rotation>; 1] = [None];
pub const ROT1: [Option<Rotation>; 2] = [None, Some(Rotation::Left)];
pub const ROT3: [Option<Rotation>; 4] = [
    None,
    Some(Rotation::Left),
    Some(Rotation::Right),
    Some(Rotation::Flip),
];

pub fn rotations(params: &Params, node: &Node, focus: Entity) -> &'static [Option<Rotation>] {
    if params.frozen[focus.qa] {
        &ROT0
    } else {
        let num_actions = node.grid[focus.qa].num_rot();
        if num_actions == 0 {
            &ROT0
        } else if num_actions == 1 {
            &ROT1
        } else {
            &ROT3
        }
    }
}

pub fn check_indy_path(params: &Params, node0: &Node) -> bool {
    let mut node = *node0;
    while node.indy.qa != params.exit {
        if !node.eval_indy_step() {
            return false;
        };
    }
    true
}
