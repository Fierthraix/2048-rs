extern crate ncurses;

mod board;
mod display;

use ncurses::*;
use board::Board;
use display::Curses;

fn main() {
    let mut board = Board::new();

    initscr();
    start_color();
    let c = Curses::new();

    // Main game loop
    loop {
        c.draw(&board);
        keyboard_input(&mut board);
        //clear();
    }
}

fn keyboard_input(b: &mut Board) {
    loop {
        let fdsa = getch();
        match fdsa as u8 as char {
            'w' | 'k' => b.up(),
            'a' | 'h' => b.left(),
            's' | 'j' => b.down(),
            'd' | 'l' => b.right(),
            // Escape character
            '\x1b' => {
                getch(); // Get rid of the '\x5b' ('[')
                match getch() as u8 as char {
                    'A' => b.up(),
                    'D' => b.left(),
                    'B' => b.down(),
                    'C' => b.right(),
                    _ => {
                        continue;
                    }
                }

            }
            _ => {
                continue;
            }
        }
        break;
    }
}
