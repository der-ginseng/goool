use std::io::{self, Write};
use rand::{thread_rng, Rng};

use crate::{braillize, split_into_parts, CellType};

#[derive(Debug)]
pub struct Game {
    cells: Vec<Vec<bool>>,
    cell_type: CellType,
    alive_color: Option<(u8, u8, u8)>,
    dead_color: Option<(u8, u8, u8)>,
    resolution: (usize, usize),
    last_two_boards: [Vec<Vec<bool>>; 2],
}

impl Game {
    pub fn new(cell_type: CellType, alive_color: Option<(u8, u8, u8)>, dead_color: Option<(u8, u8, u8)>) -> Self {
        let resolution = term_size::dimensions().unwrap();

        let mut game = Game {
            cells: vec![],
            cell_type,
            alive_color,
            dead_color,
            resolution,
            last_two_boards: [vec![], vec![]],
        };
        
        game.setup();
        game
    }

    fn setup(&mut self) {
        let (w, h) = self.calculate_board_dimensions();
        let mut rng = thread_rng();
        
        self.cells = (0..h)
            .map(|_| (0..w).map(|_| rng.gen::<bool>()).collect::<Vec<bool>>())
            .collect();
    }
    
    fn calculate_board_dimensions(&self) -> (usize, usize) {
        let (w, h) = self.resolution;

        match self.cell_type {
            CellType::Big => (w / 2, h),
            CellType::Small => (w, h * 2),
            CellType::Braille => (w * 2, h * 4),
        }
    }


    pub fn update(&mut self) {
        let current_resolution = term_size::dimensions().unwrap();
        
        if current_resolution != self.resolution || self.cells == self.last_two_boards[1] {
            self.resolution = current_resolution;
            self.setup();
        }

        self.last_two_boards[1] = self.last_two_boards[0].clone();
        self.last_two_boards[0] = self.cells.clone();

        self.cells = self.next_generation();
    }


    fn next_generation(&self) -> Vec<Vec<bool>> {
        let (w, h) = self.calculate_board_dimensions();
        let mut next_gen = vec![vec![false; w]; h];

        for i in 0..h {
            for j in 0..w {
                let neighbors = self.count_neighbors(i, j);
                next_gen[i][j] = matches!((self.cells[i][j], neighbors), (true, 2 | 3) | (false, 3));
            }
        }

        next_gen
    }


    fn count_neighbors(&self, i: usize, j: usize) -> usize {
        let mut count = 0;
        for x_offset in [-1, 0, 1] {
            for y_offset in [-1, 0, 1] {
                if (x_offset, y_offset) == (0, 0) {
                    continue;
                }

                let x = i as isize + x_offset;
                let y = j as isize + y_offset;
    
                if x >= 0 && y >= 0 
                    && (x as usize) < self.cells.len() 
                    && (y as usize) < self.cells[0].len() 
                    && self.cells[x as usize][y as usize] {
                    count += 1;
                }
            }
        }

        count
    }


    pub fn print(&self) -> Result<(), io::Error> {
        if let Some((ar, ag, ab)) = self.alive_color {
            print!("\x1B[38;2;{ar};{ag};{ab}m");
        };
        if let Some((dr, dg, db)) = self.dead_color {
            print!("\x1B[48;2;{dr};{dg};{db}m");
        };
        
        match self.cell_type {
            CellType::Big     => self.print_big_cells(),
            CellType::Small   => self.print_small_cells(),
            CellType::Braille => self.print_braille_cells(),
        }

        print!("\x1B[0m");
        io::stdout().flush()
    }

    fn print_big_cells(&self) {
        let last_row = self.cells.len() - 1;

        for (i, row) in self.cells.iter().enumerate() {
            for &cell in row {
                let cell_symbol = if cell {
                    "██"
                } else {
                    "  "
                };
                print!("{cell_symbol}");
            }
            if i != last_row {
                println!();
            }
        }
    }

    fn print_small_cells(&self) {
        let last_row = self.cells.len() / 2 - 1;

        for (i, pair) in self.cells.chunks(2).enumerate() {
            for (c1, c2) in pair[0].iter().zip(pair[1].iter()) {
                let cell_symbol = match (c1, c2) {
                    (true, true)	=> "█",
                    (false, true)	=> "▄",
                    (true, false)	=> "▀",
                    (false, false)	=> " ",
                };
                print!("{cell_symbol}");
            }
            if i != last_row {
                println!();
            }
        }
    }

    fn print_braille_cells(&self) {
        let parts = split_into_parts(self.cells.clone(), 4, 2);
        let last_row = parts.len() - 1;

        for (i, row) in parts.into_iter().enumerate() {
            for el in row {
                print!("{}", braillize(&el[..]));
            }
            if i != last_row {
                println!();
            }
        }
    }
}
