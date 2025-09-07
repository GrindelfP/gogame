pub enum Color {
    Black,
    White
}

pub struct Counter {
    white: f64,
    black: f64
}

impl Counter {
    pub fn new(size: u8) -> Self {
        match size {
            9 => Self { white: 5.5, black: 0.0 },
            13 => Self { white: 6.5, black: 0.0 },
            19 => Self { white: 7.5, black: 0.0 },
            _ => panic!("Invalid size")
        }
    }

    pub fn increment(&mut self, color: Color) {
        match color {
            Color::Black => {self.black += 1.0},
            Color::White => {self.white += 1.0},
        }
    }
}