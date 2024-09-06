use std::process::exit;
use std::time::Duration;
use std::{io, thread::sleep};

use goool::{args, game::Game};


fn main() -> Result<(), io::Error> {
    let mut args = args::Args::new();
    if let Err(err) = args.parse() {
        println!("\x1b[1;38;2;220;30;20mError:\x1b[0m \x1b[1m{}\x1b[0m", err);
        exit(1);
    }


    
    let mut game = Game::new(args.cell_type, args.alive_color, args.dead_color);

    
    loop {
        print!("\x1B[1;1H");

        game.update();

        game.print()?;
        
        sleep(Duration::from_millis(args.delay));
    }
}