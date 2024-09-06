pub mod args;
pub mod game;


#[derive(Debug, Clone)]
pub enum CellType {
    Big,
    Small,
    Braille,
}


pub fn print_full_help() {
    println!("\x1b[1mCLI Game of Life screensaver\x1b[0m");
    println!();
    println!("\x1b[1mUsage:\x1b[0m");
    println!("  goool [-c | --cell-type <cell type>] [-d | --delay <time in ms>] [--alive-color <rgb in hex>] [--dead-color <rgb in hex>]");
    println!();
    println!("\x1b[1mOptions:\x1b[0m");
    println!("  -c, --cell-type     Cell size.");
    println!("                        'big'     - 2 characters per cell.");
    println!("                        'small'   - 2 cells in 1 character.");
    println!("                        'braille' - 8 cells in 1 character.");
    println!("  -d, --delay         Delay between updates in milliseconds.");
    println!("  -ac, --alive-color  Color of alive cells (e.g. e7c27d, 8a8)");
    println!("  -dc, --dead-color   Color of alive cells (e.g. 182020, 000)");

}

pub fn color_from_hex(s: &str) -> Result<(u8, u8, u8), String> {
    let s = s.trim_matches('#');

    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..=1], 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&s[2..=3], 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&s[4..=5], 16).map_err(|e| e.to_string())?;

            Ok((r, g, b))
        },
        3 => {
            let r = u8::from_str_radix(&s[0..=0], 16).map_err(|e| e.to_string())? << 4;
            let g = u8::from_str_radix(&s[1..=1], 16).map_err(|e| e.to_string())? << 4;
            let b = u8::from_str_radix(&s[2..=2], 16).map_err(|e| e.to_string())? << 4;

            Ok((r, g, b))
        },
        _ => Err(format!("Invalid hex color: '{}'", s)),
    }

}

pub fn split_into_parts<T: Clone>(array: Vec<Vec<T>>, h: usize, w: usize) -> Vec<Vec<Vec<T>>> {
    let mut result = Vec::new();
    
    let total_rows = array.len();
    let total_cols = if total_rows > 0 { array[0].len() } else { 0 };

    for row_start in (0..total_rows).step_by(h) {
        let mut row_of_blocks = Vec::new();

        for col_start in (0..total_cols).step_by(w) {
            let mut part = Vec::new();

            for r in row_start..std::cmp::min(row_start + h, total_rows) {
                let mut sub_row = Vec::new();

                for c in col_start..std::cmp::min(col_start + w, total_cols) {
                    sub_row.push(array[r][c].clone());
                }

                if !sub_row.is_empty() {
                    part.push(sub_row);
                }
            }

            if !part.is_empty() {
                row_of_blocks.push(part.into_iter().flatten().collect::<Vec<T>>());
            }
        }

        result.push(row_of_blocks);
    }
    
    result
}


pub fn braillize(cc: &[bool]) -> char {
    let [c1, c4, c2, c5, c3, c6, c7, c8] = *cc else { unreachable!() };

    let mut cell_value = 0x2800_u32;

    if c1 { cell_value += 0x1 };
    if c2 { cell_value += 0x2 };
    if c3 { cell_value += 0x4 };
    if c4 { cell_value += 0x8 };
    if c5 { cell_value += 0x10 };
    if c6 { cell_value += 0x20 };
    if c7 { cell_value += 0x40 };
    if c8 { cell_value += 0x80 };

    let character = char::from_u32(cell_value).unwrap();

    character
}
