#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn into_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}
