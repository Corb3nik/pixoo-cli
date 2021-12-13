pub type Font = Vec<Vec<u8>>;

#[derive(Debug)]
pub struct Character {
    pub width: usize,
    pub height: usize,
    pub font: Font,
}

impl Character {
    pub fn new(font: Font) -> Self {
        let height = font.len();

        if height == 0 {
            panic!("Invalid font: missing rows");
        }

        let width = font[0].len();

        if width == 0 {
            panic!("Invalid font: missing columns");
        }

        let standard = font.iter().all(|row| row.len() == width);
        if !standard {
            panic!("Invalid font: not all rows have same length");
        }

        Self {
            width,
            height,
            font,
        }
    }
}
