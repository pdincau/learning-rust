struct Character {
    health: u16,
    level: u16,
}

impl Character {
    pub fn health(self) -> u16 {
        self.health
    }

    pub fn level(self) -> u16 {
        self.level
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            health: 1000,
            level: 1,
        }
    }
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
}
