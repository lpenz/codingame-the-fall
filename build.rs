/*!
This file bundles all rust bin and lib source code into a singlefile.rs in the
output directory.
*/

use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler = Bundler::new(
        Path::new("src/bin/episode2.rs"),
        Path::new("src/bin/singlefile.rs"),
    );
    bundler.crate_name("the_fall");
    bundler.run();
}
