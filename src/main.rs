use std::error::Error;
mod generation;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    generation::init()
}
