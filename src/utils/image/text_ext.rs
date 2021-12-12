use super::Image;
use crate::utils::Color;

impl From<&str> for Image {
    fn from(val: &str) -> Image {
        let one: Vec<Vec<u8>> = vec![
            vec![0, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];

        let mut canvas = [[Color::default(); 16]; 16];

        let character_x = 0;
        let character_y = 0;

        for (y, row) in canvas.iter_mut().enumerate() {
            for (x, mut pixel) in row.iter_mut().enumerate() {
                let x_diff = x - character_x;
                let y_diff = y - character_y;

                if y_diff >= one.len() || x_diff >= one[y_diff].len() {
                    // Draw background
                    *pixel = Color {
                        r: 0xff,
                        g: 0x00,
                        b: 0x00,
                    };
                } else {
                    let character_pixel = one[y_diff][x_diff];
                    // Draw character
                    *pixel = Color {
                        r: 0xff,
                        g: 0xff * character_pixel,
                        b: 0xff * character_pixel,
                    };
                }
            }
        }
        println!("{:#?}", canvas);

        Image::from(canvas)
    }
}
