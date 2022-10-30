#!/usr/bin/env rust-script
//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! tokei = "12.1"
//! timeit = "0.1.2"
//! ```

#[macro_use]
extern crate timeit;

use tokei::{Config, Languages};

fn main() {

    let paths = &["./resources/cpython"];
    // Exclude any path that contains any of these strings.
    let excluded = &["ignored"];
    // `Config` allows you to configure what is searched and counted.
    let config = Config::default();

    let loops = 100;

    let timing = timeit_loops!(loops, {
        let mut languages = Languages::new();
        languages.get_statistics(paths, excluded, &config);
    });

    println!{"{} loops, average time was: {} sec", loops, timing}
}