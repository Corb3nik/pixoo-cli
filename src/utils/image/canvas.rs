use super::character::Character;
use crate::utils::Color;

const CANVAS_WIDTH: usize = 16;
const CANVAS_HEIGHT: usize = 16;

pub struct Canvas {
    pixels: [[Color; 16]; 16],
    cursor_x: usize,
    cursor_y: usize,
    line_height: usize,
}

impl Canvas {
    pub fn new(line_height: usize, background_color: Color) -> Self {
        Self {
            line_height,
            pixels: [[background_color; 16]; 16],
            cursor_x: 7,
            cursor_y: 5,
        }
    }

    pub fn draw(&mut self, character: &Character, color: Color) {
        // Wrapping
        if self.cursor_x + character.width > CANVAS_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += self.line_height + 1;
        }

        if self.cursor_y > CANVAS_HEIGHT {
            return;
        }

        let pixels = &mut self.pixels;
        let cursor_x = self.cursor_x;
        let cursor_y = self.cursor_y;

        // Iterate over rows
        for (y, row) in pixels.iter_mut().enumerate() {
            // Iterate over each column of row
            for (x, mut pixel) in row.iter_mut().enumerate() {
                let in_character_box = x >= cursor_x
                    && x < cursor_x + character.width
                    && y >= cursor_y
                    && y < cursor_y + character.height;

                if in_character_box {
                    let should_draw = character.font[y - cursor_y][x - cursor_x];

                    if should_draw == 1 {
                        *pixel = color;
                    }
                }
            }
        }

        self.cursor_x += character.width;
    }

    pub fn finish(self) -> [[Color; 16]; 16] {
        self.pixels
    }
}
