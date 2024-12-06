use crossterm::terminal;
use std::io::{self, Read};

fn main() {
    terminal::enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03}\r", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }
                if c == 'q' {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}
