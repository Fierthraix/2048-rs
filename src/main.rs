extern crate ncurses;

mod board;
mod display;

use ncurses::*;
use board::Board;
use display::Screen;

fn main() {
    let mut board = Board::new();

    initscr(); // Start curses
    start_color(); // Allow colour

    let c = Screen::new();

    // Main game loop
    loop {
        c.draw(&board);
        keyboard_input(&mut board);
    }
}

fn keyboard_input(b: &mut Board) {
    loop {
        let input = getch();
        // Get movement from wasd, hjkl, or arrow keys
        match input as u8 as char {
            // Get wasd or hjkl
            'w' | 'k' => b.up(),
            'a' | 'h' => b.left(),
            's' | 'j' => b.down(),
            'd' | 'l' => b.right(),
            // Remove Escape Character
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
