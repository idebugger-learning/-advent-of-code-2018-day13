use std::{
    io::{stdout, Error, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{self, position},
    execute,
    style::Print,
    ExecutableCommand,
};
use railroad::RailRoad;

mod railroad;

fn main() -> Result<(), Error> {
    // let input = include_str!("./data/input_example.txt");
    let input = include_str!("./data/input.txt");

    let mut railroad = RailRoad::new_from_str(input);

    // print!("{}", railroad);
    while railroad.carts.len() > 1 {
        // execute!(
        //     stdout(),
        //     cursor::MoveToPreviousLine(railroad.get_display_size() as u16)
        // )?;
        railroad.tick();
        // print!("{}", railroad);
        // thread::sleep(Duration::from_secs(1));
    }

    println!("Crashed carts: {:?}", railroad.crushed_carts);
    println!("Cart left: {:?}", railroad.carts);

    Ok(())
}
