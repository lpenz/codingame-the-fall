/*!
This file bundles all rust bin and lib source code into a singlefile.rs in the
output directory.
*/

use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler = Bundler::new(
        Path::new("src/bin/the_last_crusade.rs"),
        Path::new("src/bin/singlefile.rs"),
    );
    bundler.crate_name("the_last_crusade");
    bundler.run();
}
