// #![allow(unused)] // For beginning only.

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod froms;
mod prelude;

fn main() -> Result<()> {
    println!("Hello, world!");

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }

    Ok(())
}
