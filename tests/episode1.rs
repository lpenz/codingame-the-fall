// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert::TryFrom;
use std::iter;

use anyhow::Result;

use the_fall::core::*;
use the_fall::entrypoint1::*;
use the_fall::input::*;

#[test]
fn test_input() -> Result<()> {
    let vec_lines = vec![
        "13 10",
        "3 12 8 6 2 2 8 2 9 0 0 0 0",
        "11 5 10 0 0 0 3 0 3 0 0 0 0",
        "0 11 2 2 2 2 6 2 1 2 2 13 0",
        "0 0 0 0 0 12 8 2 1 2 2 9 0",
        "0 0 12 2 2 1 4 2 10 0 0 11 13",
        "0 0 3 0 0 7 9 0 0 0 0 0 3",
        "0 0 11 2 2 10 11 2 2 2 2 2 9",
        "0 12 8 2 2 2 2 8 2 2 2 2 10",
        "0 11 4 2 2 2 2 10 12 13 12 13 0",
        "0 0 3 12 8 8 13 12 4 5 5 10 0",
        "2",
    ];
    let mut it_lines = vec_lines.iter().cloned().map(String::from).map(Ok);
    let mut params = Params::default();
    let mut node = Node::default();
    input_first(&mut it_lines, &mut params, &mut node)?;
    let inputs = vec![
        ("0 0 TOP", Qa::try_from((0, 1)).unwrap()),
        ("0 1 TOP", Qa::try_from((1, 1)).unwrap()),
        ("1 1 LEFT", Qa::try_from((1, 2)).unwrap()),
        ("1 2 TOP", Qa::try_from((2, 2)).unwrap()),
        ("2 2 LEFT", Qa::try_from((3, 2)).unwrap()),
        ("3 2 LEFT", Qa::try_from((4, 2)).unwrap()),
        ("4 2 LEFT", Qa::try_from((5, 2)).unwrap()),
        ("5 2 LEFT", Qa::try_from((6, 2)).unwrap()),
        ("6 2 LEFT", Qa::try_from((7, 2)).unwrap()),
        ("7 2 LEFT", Qa::try_from((8, 2)).unwrap()),
        ("8 2 LEFT", Qa::try_from((8, 3)).unwrap()),
        ("8 3 TOP", Qa::try_from((8, 4)).unwrap()),
        ("8 4 TOP", Qa::try_from((7, 4)).unwrap()),
        ("7 4 RIGHT", Qa::try_from((6, 4)).unwrap()),
        ("6 4 RIGHT", Qa::try_from((6, 5)).unwrap()),
        ("6 5 TOP", Qa::try_from((6, 6)).unwrap()),
        ("6 6 TOP", Qa::try_from((7, 6)).unwrap()),
        ("7 6 LEFT", Qa::try_from((8, 6)).unwrap()),
        ("8 6 LEFT", Qa::try_from((9, 6)).unwrap()),
        ("9 6 LEFT", Qa::try_from((10, 6)).unwrap()),
        ("9 7 RIGHT", Qa::try_from((8, 7)).unwrap()),
        ("8 7 RIGHT", Qa::try_from((7, 7)).unwrap()),
        ("7 7 RIGHT", Qa::try_from((7, 8)).unwrap()),
        ("7 8 TOP", Qa::try_from((6, 8)).unwrap()),
        ("6 8 RIGHT", Qa::try_from((5, 8)).unwrap()),
        ("5 8 RIGHT", Qa::try_from((4, 8)).unwrap()),
        ("4 8 RIGHT", Qa::try_from((3, 8)).unwrap()),
        ("3 8 RIGHT", Qa::try_from((2, 8)).unwrap()),
        ("2 8 RIGHT", Qa::try_from((2, 9)).unwrap()),
    ];
    for i in &inputs {
        input_ep1(&mut iter::once(Ok(i.0.to_string())), &mut params, &mut node)?;
        assert_eq!(eval(&params, &node), i.1);
    }
    Ok(())
}
