#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Melee,
    Ranged,
}

const MELEE_MAX_DISTANCE: u16 = 2;
const RANGED_MAX_DISTANCE: u16 = 5;

impl Style {
    pub fn range(&self) -> u16 {
        match *self {
            Style::Melee => MELEE_MAX_DISTANCE,
            Style::Ranged => RANGED_MAX_DISTANCE,
        }
    }
}
