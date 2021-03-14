#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Melee,
    Ranged,
}

impl Style {
    pub fn range(&self) -> u16 {
        match *self {
            Style::Melee => 2,
            Style::Ranged => 5,
        }
    }
}
