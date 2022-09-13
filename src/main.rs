#![allow(unused_imports, unused_variables, unused_mut)]

use std::error::Error;
mod generation;
use generation::*;
mod ui;
use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    generation::init()
}
