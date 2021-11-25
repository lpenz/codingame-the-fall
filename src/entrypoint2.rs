// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::BufRead;

use crate::core::*;
use crate::input::*;

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

pub fn solve(params: &Params, node: &Node) -> Option<Action> {
    let mut stepsbase = VecDeque::new();
    let focus = node.indy.step(node)?;
    if !solve_helper(params, node, focus, 0, &mut stepsbase) {
        return None;
    }
    // Check rocks:
    Some(stepsbase[0])
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
