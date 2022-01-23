// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::VecDeque;
use std::fmt;

use crate::andex::*;

pub const MAX_WIDTH: u16 = 20;
pub const MAX_HEIGHT: u16 = 20;

pub type Sqrid = crate::sqrid_create!(MAX_WIDTH, MAX_HEIGHT, false);
pub type Qa = crate::qa_create!(Sqrid);
pub type Qr = crate::Qr;
pub type Gridbool = crate::gridbool_create!(Sqrid);

/* Cell *************************************************************/

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
        write!(
            f,
            "{}",
            match self {
                Cell::Type0 => "  ",
                Cell::Type1 => "\u{254b}\u{2501}",
                Cell::Type2 => "\u{2501}\u{2501}",
                Cell::Type3 => "\u{2503} ",
                Cell::Type4 => "\u{251b}\u{250f}",
                Cell::Type5 => "\u{2513}\u{2517}",
                Cell::Type6 => "\u{253b}\u{2501}",
                Cell::Type7 => "\u{2523}\u{2501}",
                Cell::Type8 => "\u{2533}\u{2501}",
                Cell::Type9 => "\u{252b}\u{2501}",
                Cell::Type10 => "\u{251b}\u{2501}",
                Cell::Type11 => "\u{2517}\u{2501}",
                Cell::Type12 => "\u{250f}\u{2501}",
                Cell::Type13 => "\u{2513}\u{2501}",
            }
        )
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

/* Rotation *********************************************************/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    Left,
    Right,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rotation::Left => write!(f, "LEFT"),
            Rotation::Right => write!(f, "RIGHT"),
        }
    }
}

/* Entity ***********************************************************/

#[derive(Debug, Default, Clone, Copy)]
pub struct Entity {
    pub qa: Qa,
    pub qr: Qr,
}

impl Entity {
    pub fn step(&self, node: &Node) -> Option<Entity> {
        let cell = node.grid[self.qa];
        if let Some(qr) = cell.enter(&self.qr) {
            if let Some(qa) = self.qa + qr {
                return Some(Entity { qa, qr });
            }
        }
        None
    }
    pub fn iter<'a>(&self, node: &'a Node) -> EntityPathIter<'a> {
        EntityPathIter {
            entity: Some(*self),
            node,
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.qa.tuple();
        write!(f, "({} {}, {})", t.0, t.1, self.qr)
    }
}

impl AsRef<Qa> for Entity {
    fn as_ref(&self) -> &Qa {
        &self.qa
    }
}

pub struct EntityPathIter<'a> {
    pub entity: Option<Entity>,
    pub node: &'a Node,
}

impl<'a> Iterator for EntityPathIter<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ent0) = self.entity {
            self.entity = ent0.step(self.node);
        }
        self.entity
    }
}

/* Action ***********************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Wait,
    Rotate { qa: Qa, rot: Rotation },
}

impl Action {
    pub fn new(qa: Qa, rot: Rotation) -> Action {
        Action::Rotate { qa, rot }
    }

    pub fn qa(&self) -> Option<Qa> {
        match self {
            Action::Rotate { qa, rot: _ } => Some(*qa),
            _ => None,
        }
    }

    pub fn rotation(&self) -> Option<Rotation> {
        match self {
            Action::Rotate { qa: _, rot } => Some(*rot),
            _ => None,
        }
    }

    pub fn available(params: &Params, node: &Node, focus: &Entity) -> Vec<Vec<Action>> {
        let mut ret = vec![vec![]];
        if !params.frozen[focus.qa] {
            let num_rot = node.grid[focus.qa].num_rot();
            if num_rot > 0 {
                ret.push(vec![Action::new(focus.qa, Rotation::Left)]);
            }
            if num_rot > 1 {
                ret.push(vec![Action::new(focus.qa, Rotation::Right)]);
                ret.push(vec![
                    Action::new(focus.qa, Rotation::Right),
                    Action::new(focus.qa, Rotation::Right),
                ]);
            }
        }
        ret
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

/* Rocks: Andex *****************************************************/

pub enum IRockMarker {}
pub type IRock = Andex<IRockMarker, 10>;
pub type Rocks = AndexableArray<IRock, Option<Entity>, { IRock::SIZE }>;

/* Params, Node *****************************************************/

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
    pub rock: Rocks,
}

impl Node {
    pub fn apply(&mut self, action: &Action) {
        match action {
            Action::Wait => {}
            Action::Rotate { qa, rot } => {
                self.grid[qa] = self.grid[qa].rotate(rot);
            }
        }
    }
    pub fn eval_indy_step(&mut self) -> bool {
        if let Some(indy) = self.indy.step(self) {
            self.indy = indy;
            return true;
        }
        false
    }
    pub fn eval_all_step(&mut self) -> bool {
        if !self.eval_indy_step() {
            return false;
        }
        for irock in IRock::iter() {
            if let Some(rock) = self.rock[irock] {
                self.rock[irock] = rock.step(self);
            }
        }
        true
    }
    pub fn has_rock_collision(&self) -> Option<IRock> {
        for irock in IRock::iter() {
            if let Some(rock) = self.rock[irock] {
                if rock.qa == self.indy.qa {
                    return Some(irock);
                }
            }
        }
        None
    }
}

pub fn check_indy_path(params: &Params, node: &Node, indy: &Entity) -> bool {
    if indy.qa == params.exit {
        return true;
    }
    for indy in indy.iter(node) {
        if indy.qa == params.exit {
            return true;
        }
    }
    false
}

// Complete simulation

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Destiny {
    InvalidAction,
    Wall(Qa),
    Rock(IRock, Qa),
    Victory,
}

impl Destiny {
    pub fn irock(&self) -> Option<IRock> {
        match self {
            Destiny::Rock(irock, _) => Some(*irock),
            _ => None,
        }
    }
}

pub fn simulate(params: &Params, node0: &Node, mut steps: VecDeque<Action>) -> Destiny {
    let mut node = *node0;
    while node.indy.qa != params.exit {
        if let Some(action) = steps.pop_front() {
            if action.qa() == Some(node.indy.qa) {
                return Destiny::InvalidAction;
            }
            node.apply(&action);
        }
        if !node.eval_all_step() {
            return Destiny::Wall(node.indy.qa);
        }
        if let Some(irock) = node.has_rock_collision() {
            return Destiny::Rock(irock, node.indy.qa);
        }
    }
    Destiny::Victory
}
