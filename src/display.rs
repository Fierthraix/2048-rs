use std::process;

use board::{Board, SIZE};

use ncurses::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Number {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    NS,
}

impl Number {
    fn new(num: usize) -> Number {
        match num {
            0 => Number::N0,
            1 => Number::N1,
            2 => Number::N2,
            3 => Number::N3,
            4 => Number::N4,
            5 => Number::N5,
            6 => Number::N6,
            7 => Number::N7,
            8 => Number::N8,
            9 => Number::N9,
            _ => Number::NS,
        }
    }
    fn val(self) -> [u8; 15] {
        match self {
            Number::N0 => [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
            Number::N1 => [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            Number::N2 => [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
            Number::N3 => [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            Number::N4 => [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1],
            Number::N5 => [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            Number::N6 => [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
            Number::N7 => [1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
            Number::N8 => [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
            Number::N9 => [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            Number::NS => [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    fn formatted(num: usize) -> [Number; SIZE] {
        let mut nums = [Number::NS; SIZE];
        if num != 0 {
            let mut i;
            match num.to_string().len() {
                1 | 2 => i = 1,
                3 | 4 => i = 0,
                _ => return nums,
            }
            for numchar in num.to_string().chars() {
                nums[i] = Number::new(numchar.to_digit(10).unwrap() as usize);
                i += 1;
            }
        }
        nums
    }
}

/// Wrapper to print a string at a given x and y with certain attributes
fn mvaddstr_attr(y: usize, x: usize, s: &str, attr: u32) {
    attron(attr as u32);
    mvaddstr(y as i32, x as i32, s);
    attroff(attr as u32);
}

pub struct Screen {
    colours: Vec<attr_t>,

    //TODO: change these to constants of some sort
    foreground: u32,
    frame: u32,
    background: u32,
    //min_x: usize,
    //min_y: usize,
    tile_height: usize,
    tile_width: usize,
}

impl Screen {
    pub fn new() -> Self {
        let mut attrs = Screen::get_attrs();
        Screen {
            foreground: attrs[12],
            frame: attrs[13],
            background: attrs[14],
            // TODO replace this
            tile_height: 7,
            tile_width: 17,
            colours: {
                attrs.truncate(12);
                attrs
            },
        }
    }
    fn get_attrs() -> Vec<attr_t> {
        start_color();
        let mut color_list = Vec::with_capacity(16);
        if has_colors() {
            let colours = if COLORS() != 256 {
                // Using standard 16 colours
                vec![
                    (COLOR_BLACK, COLOR_BLACK), //0
                    (COLOR_BLACK, COLOR_WHITE), //2
                    (COLOR_BLACK, COLOR_CYAN), //4
                    (COLOR_BLACK, COLOR_BLUE), //8
                    (COLOR_BLACK, COLOR_GREEN), //16
                    (COLOR_BLACK, COLOR_YELLOW), //32
                    (COLOR_BLACK, COLOR_MAGENTA), //64
                    (COLOR_BLACK, COLOR_RED), //12r
                    (COLOR_BLACK, COLOR_RED), //256
                    (COLOR_BLACK, COLOR_RED), //512
                    (COLOR_BLACK, COLOR_RED), //1024
                    (COLOR_BLACK, COLOR_RED), //2048
                    (COLOR_BLACK, COLOR_BLACK), //dark fg
                    (COLOR_BLACK, COLOR_BLACK), //frame
                    (COLOR_BLACK, COLOR_BLACK), //back
                ]
            } else {
                // Use 256 colors when available
                vec![
                    (0, 240),
                    (0, 231),
                    (0, 229),
                    (0, 215),
                    (7, 209),
                    (7, 203),
                    (7, 196),
                    (7, 222),
                    (7, 227),
                    (7, 226),
                    (7, 214),
                    (7, 9),
                    (0, 234),
                    (0, 240),
                    (0, 250),
                ]
            };
            for i in 0..colours.len() {
                init_pair(i as i16, colours[i as usize].0, colours[i as usize].1);
                color_list.push(COLOR_PAIR(i as i16));
            }
        } else {
            // If there are no colors use the black/white default
            for i in 0..16 {
                if i == 0 {
                    color_list.push(A_NORMAL());
                } else {
                    color_list.push(A_REVERSE());
                }
            }
        }
        color_list
    }
    /// Draw the gameboard on the screen
    pub fn draw(&self, b: &Board) {
        let (score, board, game_over) = b.current_state();

        self.draw_frame();

        // Draw each individual tile
        #[allow(clippy::needless_range_loop)]
        for i in 0..SIZE {
            for j in 0..SIZE {
                let x = (j * (self.tile_width + 1)) + 1;
                let y = (i * (self.tile_height + 1)) + 1;
                self.draw_tile(x, y, board[i][j]);
            }
        }
        mvaddstr(33, 40, format!("SCORE: {}", score).as_ref());

        if game_over {
            //TODO: q to quit, enter to restart
            // Print Game Over
            mvaddstr(33, 5, "GAME OVER (press 'q' to quit)");
            // Exit game when enter is hit
            loop {
                if let 'q' = getch() as u8 as char {
                    process::exit(0)
                }
            }
        }
    }
    /// Draw the frame for the entire board
    fn draw_frame(&self) {
        for x in 0..SIZE + 1 {
            for y in 0..SIZE + 1 {
                let px = x * (self.tile_width + 1);
                let py = y * (self.tile_height + 1);

                if x < SIZE {
                    for ppx in px..(px + self.tile_width + 2) {
                        mvaddstr_attr(py, ppx, " ", self.frame);
                    }
                }

                if y < SIZE {
                    for ppy in py..(py + self.tile_height + 1) {
                        mvaddstr_attr(ppy, px, " ", self.frame);
                    }
                }

            }
        }
    }
    /// Draw a tile with the appropriate number inside
    fn draw_tile(&self, x: usize, y: usize, val: usize) {
        for py in y..(y + self.tile_height) {
            for px in x..(x + self.tile_width) {
                mvaddstr_attr(py, px, " ", self.get_colour_pair(val));
            }
        }
        // Add appropriate padding to the number
        if val != 0 {
            let charnums = Number::formatted(val);

            let j = y + ((self.tile_height - 5) / 2);
            for (i, charnum) in charnums.iter().enumerate() {
                self.draw_number(x + (i * 4), j, *charnum, self.get_colour_pair(val));
            }

            //TODO: Draw the last row

        }
    }
    /// Draw a number using its bitmap and attributes at a given position
    fn draw_number(&self, x: usize, y: usize, num: Number, attr: u32) {
        let mut bit = 0;

        for i in 0..SIZE {
            for j in 0..5 {
                if i != 0 {
                    bit = num.val()[j * 3 + (i - 1)];
                }
                if bit == 1 {
                    mvaddstr_attr(y + j, x + i, " ", self.foreground);
                } else {
                    mvaddstr_attr(y + j, x + i, " ", attr);
                }
            }
        }
    }
    /// Get the colour-pair combo for a tile of a given value
    fn get_colour_pair(&self, val: usize) -> u32 {
        if val == 0 {
            return self.background;
        }
        for i in (0..12).rev() {
            if (val >> i) > 0 {
                return self.colours[i];
            }
        }
        self.colours[0]
    }
}

#[test]
fn test_number_formatted() {
    //0
    assert_eq!(
        Number::formatted(0),
        [Number::NS, Number::NS, Number::NS, Number::NS]
    );
    //1 digit
    assert_eq!(
        Number::formatted(7),
        [Number::NS, Number::N7, Number::NS, Number::NS]
    );
    //2 digit
    assert_eq!(
        Number::formatted(69),
        [Number::NS, Number::N6, Number::N9, Number::NS]
    );
    //3 digit
    assert_eq!(
        Number::formatted(171),
        [Number::N1, Number::N7, Number::N1, Number::NS]
    );
    //4 digit
    assert_eq!(
        Number::formatted(9696),
        [Number::N9, Number::N6, Number::N9, Number::N6]
    );
    //4+ digit
    assert_eq!(
        Number::formatted(72727),
        [Number::NS, Number::NS, Number::NS, Number::NS]
    );
}
