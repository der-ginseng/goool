use std::time::Duration;
use std::{io, thread::sleep};

use clap::Parser;

use goool::{CellType, color_from_hex};
use goool::game::Game;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    cell_type: CellType,

	#[arg(long)]
    alive_color: Option<String>,

    #[arg(long)]
    dead_color: Option<String>,

    #[arg(short, long, default_value_t = 100)]
    delay: u64,
}


fn main() -> Result<(), io::Error> {
    run()
}

fn run() -> Result<(), io::Error> {
	let args = Args::parse();

    let alive_color = args.alive_color.map(|color_hex| {
        color_from_hex(&color_hex).unwrap_or_else(|| {
            eprintln!("{} is invalid. Please use a valid hex string (e.g., #ffffff).", color_hex);
            std::process::exit(1)
        })
    });

    let dead_color = args.dead_color.map(|color_hex| {
        color_from_hex(&color_hex).unwrap_or_else(|| {
            eprintln!("{} is invalid. Please use a valid hex string (e.g., #ffffff).", color_hex);
            std::process::exit(1)
        })
    });


    
	let mut game = Game::new(args.cell_type, alive_color, dead_color);

    
    loop {
        print!("\x1B[1;1H");

        game.update();

        game.print()?;
        
        sleep(Duration::from_millis(args.delay));
    }
}