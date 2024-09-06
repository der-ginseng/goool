use std::{env, process::exit};

use crate::{color_from_hex, print_full_help, CellType};

#[derive(Debug)]
pub struct Args {
    pub alive_color: Option<(u8, u8, u8)>,
    pub dead_color: Option<(u8, u8, u8)>,
    pub delay: u64,
    pub cell_type: CellType,
}

impl Args {
    pub fn new() -> Self {
        // Default values
        Self { 
            alive_color: None, 
            dead_color: None, 
            delay: 100, 
            cell_type: CellType::Small 
        }
    }


    pub fn parse(&mut self) -> Result<(), String> {
        let mut cmd_args = env::args().skip(1);

        while let Some(arg) = cmd_args.next() {
            if arg == "-h" || arg == "--help" {
                print_full_help();
                exit(0)
            }

            if !arg.starts_with('-') {
                return Err(format!("Unexpected argument: '{}'\nA flag was expected", arg))
            }

            if let Some(value) = cmd_args.next() {
                if value.starts_with('-') || value.is_empty() {
                    return Err(format!("Incorrect argument value: '{}'", value))
                }

                self.set_value_by_flag(&arg, &value)?
            }
            else {
                return Err(format!("Missing value for argument: '{}'", arg))
            }
        }

        Ok(())
    }

    fn set_value_by_flag(&mut self, flag: &str, value: &str) -> Result<(), String> {
        match flag {
            "-c"  | "--cell-type"   => self.cell_type   = parse_cell_type(value)?,
            "-d"  | "--delay"       => self.delay       = parse_delay(value)?,
            "-ac" | "--alive-color" => self.alive_color = Some(parse_color(value)?),
            "-dc" | "--dead-color"  => self.dead_color  = Some(parse_color(value)?),

            unknown => return Err(format!("Unknown flag: '{}'", unknown)),
        }

        Ok(())
    }
}

fn parse_cell_type(s: &str) -> Result<CellType, String> {
    match s {
        "big"     => Ok(CellType::Big),
        "small"   => Ok(CellType::Small),
        "braille" => Ok(CellType::Braille),
        
        unknown => Err(format!("Unknown cell type: '{}'", unknown)),
    }
}

fn parse_delay(s: &str) -> Result<u64, String> {
    s.parse().map_err(|_| format!("Invalid number: '{}'", s))
}

fn parse_color(s: &str) -> Result<(u8, u8, u8), String> {
    color_from_hex(s)
}