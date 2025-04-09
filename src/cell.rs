#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn is_dead(&self) -> bool {
        *self == Self::Dead
    }

    pub fn is_alive(&self) -> bool {
        *self == Self::Alive
    }

    pub fn die(&mut self) -> &mut Self {
        *self = Self::Dead;
        self
    }

    pub fn revive(&mut self) -> &mut Self {
        *self = Self::Alive;
        self
    }
}
