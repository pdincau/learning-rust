use crate::rpg::State::Alive;

struct Character {
    health: u16,
    level: u16,
    state: State,
}

impl Character {
    pub fn health(self) -> u16 {
        self.health
    }

    pub fn level(self) -> u16 {
        self.level
    }

    pub fn state(self) -> State {
        self.state
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            health: 1000,
            level: 1,
            state: Alive,
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn health_starts_at_1000() {
        let character = Character::default();

        assert_eq!(1000, character.health());
    }

    #[test]
    fn level_starts_at_1() {
        let character = Character::default();

        assert_eq!(1, character.level());
    }

    #[test]
    fn starts_as_alive() {
        let character = Character::default();

        assert_eq!(Alive, character.state());
    }
}