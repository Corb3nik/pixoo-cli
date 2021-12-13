use super::character_definitions::CHARACTERS;
use super::Canvas;
use super::Image;
use crate::utils::Color;

static LINE_HEIGHT: usize = 5;

impl From<&str> for Image {
    fn from(val: &str) -> Image {
        let background_color = Color {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        };

        let mut canvas = Canvas::new(LINE_HEIGHT, background_color);

        let characters = val.chars().map(|c| {
            CHARACTERS
                .get(&c)
                .expect("Font does not exist for character")
        });

        let text_color = Color {
            r: 0x00,
            g: 0xff,
            b: 0x00,
        };

        for character in characters {
            canvas.draw(character, text_color);
        }

        let pixels = canvas.finish();
        let image = Image::from(pixels);
        println!("{}", image);
        image
    }
}
