// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::BufRead;

use super::core::*;
use super::input::*;

pub fn solve_helper(
    params: &Params,
    node0: &Node,
    focus0: Entity,
    iturn: usize,
    steps: &mut VecDeque<Action>,
) -> bool {
    if check_indy_path(params, node0, &focus0) {
        eprintln!("SUCCESS");
        steps.push_front(Action::Wait);
        return true;
    }
    for actions in Action::available(params, node0, &focus0) {
        let mut node = *node0;
        for a in &actions {
            node.apply(a);
        }
        if let Some(focus) = focus0.step(&node) {
            if solve_helper(params, &node, focus, iturn + 1, steps) {
                for a in &actions {
                    steps.push_front(*a);
                }
                return true;
            }
        }
    }
    false
}

pub fn rock_solve(
    params: &Params,
    node: &Node,
    mut steps: VecDeque<Action>,
    irock: IRock,
    qa_collision: Qa,
) -> Option<Action> {
    for focus in node.rock[irock].unwrap().iter(node) {
        if focus.qa == qa_collision {
            return None;
        }
        if params.frozen[focus.qa] || node.grid[focus.qa].num_rot() == 0 {
            continue;
        }
        let action = Action::new(focus.qa, Rotation::Left);
        steps[0] = action;
        if simulate(params, node, steps.clone()).irock() != Some(irock) {
            return Some(action);
        }
    }
    None
}

pub fn solve(params: &Params, node: &Node) -> Option<Action> {
    let mut stepsbase = VecDeque::new();
    let focus = node.indy.step(node)?;
    if !solve_helper(params, node, focus, 0, &mut stepsbase) {
        return None;
    }
    stepsbase.push_front(Action::Wait);
    match simulate(params, node, stepsbase.clone()) {
        Destiny::Victory => Some(Action::Wait),
        Destiny::Rock(irock, qa) => rock_solve(params, node, stepsbase, irock, qa),
        _ => Some(stepsbase[1]),
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut params = Params::default();
    let mut node = Node::default();
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    input_first(&mut stdin_lines, &mut params, &mut node)?;
    eprintln!(
        "Dimensions: {}x{}; exit {}",
        params.width, params.height, params.exit
    );
    loop {
        input_ep2(&mut stdin_lines, &params, &mut node)?;
        if let Some(action) = solve(&params, &node) {
            println!("{}", action);
            node.apply(&action);
        } else {
            eprintln!("could not find solution");
        }
    }
}
