// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error::Error;
use std::io;
use std::io::BufRead;

use crate::core::*;
use crate::input::*;

pub fn eval(params: &Params, node: &Node) -> Qa {
    let dir = params.grid[node.indy]
        .enter(node.dir)
        .expect("invalid indy direction");
    (node.indy + dir).expect("invalid next indy direction")
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
        input(&mut stdin_lines, &params, &mut node)?;
        let qa = eval(&params, &node);
        let t = qa.tuple();
        println!("{} {}", t.0, t.1);
    }
}
