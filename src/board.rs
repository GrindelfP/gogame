#[derive(Debug)]
/// aaa
pub struct Board {
    width: u8,
    height: u8,
    elements: Vec<Vec<u8>>
}

impl Board {
    pub(crate) fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            elements: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub(crate) fn update_at(&mut self, position: (usize, usize), update: u8) {
        self.elements[position.0][position.1] = update;
    }
}
