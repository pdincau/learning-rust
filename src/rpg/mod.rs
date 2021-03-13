use crate::rpg::State::{Alive, Dead};

#[derive(Copy, Clone)]
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

    pub fn deal_damage(self, character: &mut Character, amount: u16) {
        character.receive_damage(amount)
    }

    fn receive_damage(&mut self, amount: u16) {
        if amount >= self.health {
            self.state = Dead;
            self.health = 0;
        } else {
            self.health -= amount
        }
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::rpg::State::Dead;

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

    #[test]
    fn deals_damage() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 10);

        assert_eq!(990, attackee.health());
    }

    #[test]
    fn dies_when_damage_equals_health() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 1000);

        assert_eq!(0, attackee.health());
        assert_eq!(Dead, attackee.state());
    }
}
