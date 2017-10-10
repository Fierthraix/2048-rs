use board::{Board, SIZE};

use ncurses::*;

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
    fn new(num: isize) -> Number {
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
    fn val(&self) -> [u8; 15] {
        match *self {
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
}

fn mv_add_attribute_str_yx(y: usize, x: usize, s: &str, attr: u32) {
    attron(attr as u32);
    mvaddstr(y as i32, x as i32, s);
    attroff(attr as u32);
}

pub struct Curses {
    pub Colours: Vec<attr_t>,

    //TODO: change these to constants of some sort
    pub DARK_FOREGROUND: u32,
    pub LIGHT_FOREGROUND: u32,
    pub FRAME: u32,
    pub BACKGROUND: u32,
    pub MIN_X: usize,
    pub MIN_Y: usize,
    pub TILE_DIMENSIONS: (usize, usize),
}

impl Curses {
    pub fn new() -> Curses {
        let attrs = Curses::get_attrs();
        Curses {
            DARK_FOREGROUND: attrs[12],
            LIGHT_FOREGROUND: attrs[13],
            FRAME: attrs[14],
            BACKGROUND: attrs[15],
            MIN_X: 80,
            MIN_Y: 24,
            // TODO replace this
            TILE_DIMENSIONS: (17, 10),
            Colours: attrs,
        }
    }
    fn get_attrs() -> Vec<attr_t> {
        let mut color_list = Vec::with_capacity(16);
        if has_colors() {
            let colours;
            if COLORS() != 256 {
                // Using standard 16 colours
                colours = [
                    (COLOR_BLACK, COLOR_BLACK), //0
                    (COLOR_BLACK, COLOR_WHITE), //2
                    (COLOR_BLACK, COLOR_CYAN), //4
                    (COLOR_BLACK, COLOR_BLUE), //8
                    (COLOR_BLACK, COLOR_GREEN), //16
                    (COLOR_BLACK, COLOR_YELLOW), //32
                    (COLOR_BLACK, COLOR_MAGENTA), //64
                    (COLOR_BLACK, COLOR_RED), //128
                    (COLOR_BLACK, COLOR_RED), //256
                    (COLOR_BLACK, COLOR_RED), //512
                    (COLOR_BLACK, COLOR_RED), //1024
                    (COLOR_BLACK, COLOR_RED), //2048
                    (COLOR_BLACK, COLOR_BLACK), //dark fg
                    (COLOR_BLACK, COLOR_BLACK), //light fg
                    (COLOR_BLACK, COLOR_BLACK), //frame
                    (COLOR_BLACK, COLOR_BLACK), //back
                ];
            } else {
                // Use 256 colors when available
                colours = [
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
                    (0, 250),
                    (0, 240),
                    (0, 250),
                ];
            }
            for i in 0..16 {
                init_pair(i as i16, colours[i as usize].0, colours[i as usize].1);
                color_list.push(COLOR_PAIR(i));
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

    fn draw_frame(&self, b: &Board) {
        for x in 0..5 {
            for y in 0..5 {
                let px = x * (self.TILE_DIMENSIONS.0 + 1);
                let py = y * (self.TILE_DIMENSIONS.1 + 1);

                if x < 4 {
                    for ppx in px..(px + self.TILE_DIMENSIONS.0 + 2) {
                        mv_add_attribute_str_yx(py, ppx, " ", self.FRAME);
                    }
                }

                if y < 4 {
                    for ppy in py..(py + self.TILE_DIMENSIONS.1 + 1) {
                        mv_add_attribute_str_yx(ppy, px, " ", self.FRAME);
                    }
                }

            }
        }
    }
    fn draw_number(&self, x: usize, y: usize, num: Number, attr: u32) {
        let mut bit = 0;

        for i in 0..4 {
            for j in 0..5 {
                if i != 0 {
                    bit = num.val()[j * 3 + (i - 1)];
                }
                if bit == 1 {
                    mv_add_attribute_str_yx(y + j, x + i, " ", self.DARK_FOREGROUND);
                } else {
                    mv_add_attribute_str_yx(y + j, x + i, " ", attr);
                }
            }
        }
    }
    fn draw_tile(&self, x: usize, y: usize, val: usize) {
        for py in y..(y + self.TILE_DIMENSIONS.1) {
            for px in x..(x + self.TILE_DIMENSIONS.0) {
                mv_add_attribute_str_yx(py, px, " ", self.get_colour_pair(val));
            }
        }
        if val != 0 {}
    }
    fn get_colour_pair(&self, val: usize) -> u32 {
        if val == 0 {
            return self.BACKGROUND;
        }
        for i in (0..11).rev() {
            if (val >> i) > 0 {
                return self.Colours[i];
            }
        }
        return self.Colours[0];
    }
}
