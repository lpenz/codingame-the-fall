// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error::Error;

extern crate the_fall;
use the_fall::entrypoint1;

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint1::main()?;
    Ok(())
}
