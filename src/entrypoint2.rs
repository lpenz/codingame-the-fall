// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error::Error;
use std::io;
use std::io::BufRead;

use crate::core::*;
use crate::input::*;

pub fn solve_helper(params: &Params, node0: &Node, focus0: Entity) -> (bool, Action) {
    if check_indy_path(params, node0) {
        return (true, Action::Wait);
    }
    let rotopts = rotations(params, node0, focus0);
    for rotopt in rotopts {
        let mut node = *node0;
        let action = Action::new(focus0.qa, *rotopt);
        node.indy = focus0;
        node.apply(action);
        if let Some(focus) = node.entity_step(&focus0) {
            let (ok, subaction) = solve_helper(params, &node, focus);
            if ok {
                if rotopt.is_none() {
                    return (true, subaction);
                } else {
                    return (true, action);
                }
            }
        }
    }
    (false, Action::Wait)
}

pub fn solve(params: &Params, node0: &Node) -> Option<Action> {
    let mut node = *node0;
    // Step indy and use it as focus
    assert!(node.eval_indy_step());
    let solution = solve_helper(params, &node, node.indy);
    if solution.0 {
        Some(solution.1)
    } else {
        None
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
            node.apply_real(action);
        } else {
            eprintln!("could not find solution");
        }
    }
}
