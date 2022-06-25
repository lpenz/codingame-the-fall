// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;

use the_fall::core::*;
use the_fall::entrypoint2::*;
use the_fall::input::*;

#[test]
fn test_case4() -> Result<()> {
    let vec_lines = vec![
        "13 10",
        "-3 12 8 6 3 2 7 2 7 0 0 0 0",
        "11 5 13 0 0 0 3 0 3 0 0 0 0",
        "0 11 2 2 3 3 8 2 -9 2 3 13 0",
        "0 0 0 0 0 12 8 3 1 3 2 7 0",
        "0 0 11 2 3 1 5 2 10 0 0 11 13",
        "0 0 3 0 0 6 8 0 0 0 0 0 2",
        "0 0 11 3 3 10 11 2 3 2 3 2 8",
        "0 12 6 3 2 3 3 6 3 3 2 3 12",
        "0 11 4 2 3 2 2 11 12 13 13 13 0",
        "0 0 -3 12 7 8 13 13 4 5 4 10 0",
        "2",
    ];
    let mut it_lines = vec_lines.iter().cloned().map(String::from).map(Ok);
    let mut params = Params::default();
    let mut node = Node::default();
    input_first(&mut it_lines, &mut params, &mut node)?;
    let inputs = vec![
        vec!["0 0 TOP", "0"],
        vec!["0 1 TOP", "0"],
        vec!["1 1 LEFT", "0"],
        vec!["1 2 TOP", "0"],
        vec!["2 2 LEFT", "0"],
        vec!["3 2 LEFT", "0"],
        vec!["4 2 LEFT", "0"],
        vec!["5 2 LEFT", "0"],
        vec!["6 2 LEFT", "0"],
        vec!["7 2 LEFT", "0"],
        vec!["8 2 LEFT", "0"],
        vec!["8 3 TOP", "0"],
        vec!["8 4 TOP", "0"],
        vec!["7 4 RIGHT", "0"],
        vec!["6 4 RIGHT", "0"],
        vec!["6 5 TOP", "0"],
        vec!["6 6 TOP", "0"],
        vec!["7 6 LEFT", "0"],
        vec!["8 6 LEFT", "0"],
        vec!["9 6 LEFT", "0"],
        vec!["10 6 LEFT", "0"],
        vec!["11 6 LEFT", "0"],
        vec!["12 6 LEFT", "0"],
        vec!["12 7 TOP", "0"],
        vec!["11 7 RIGHT", "0"],
        vec!["10 7 RIGHT", "0"],
        vec!["9 7 RIGHT", "0"],
        vec!["8 7 RIGHT", "0"],
        vec!["7 7 RIGHT", "0"],
        vec!["6 7 RIGHT", "0"],
        vec!["5 7 RIGHT", "0"],
        vec!["4 7 RIGHT", "0"],
        vec!["3 7 RIGHT", "0"],
        vec!["2 7 RIGHT", "0"],
        vec!["1 7 RIGHT", "0"],
        vec!["1 8 TOP", "0"],
        vec!["2 8 LEFT", "0"],
    ];
    for i in &inputs {
        input_ep2(
            &mut i.into_iter().map(|s| Ok(s.to_string())),
            &mut params,
            &mut node,
        )?;
        let actionopt = solve(&params, &node);
        assert!(actionopt.is_some());
        node.apply(&actionopt.unwrap());
    }
    Ok(())
}
