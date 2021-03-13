struct Character {
    health: u32
}

impl Character {
    pub fn health(self) -> u32 {
        self.health
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            health: 1000,
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
}
