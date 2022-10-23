// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error::Error;

extern crate the_fall;
pub use self::the_fall::*;

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint2::main()?;
    Ok(())
}
