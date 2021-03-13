use crate::rpg::State::{Alive, Dead};

#[derive(Copy, Clone)]
struct Character {
    level: u16,
    state: State,
}

impl Character {
    pub fn health(self) -> u16 {
        match self.state {
            State::Alive { life } => life,
            Dead => 0,
        }
    }

    pub fn level(self) -> u16 {
        self.level
    }

    pub fn status(self) -> State {
        self.state
    }

    pub fn deal_damage(self, character: &mut Character, amount: u16) {
        character.receive_damage(amount)
    }

    pub fn heal(self, character: &mut Character) {
        character.receive_healing()
    }

    fn receive_damage(&mut self, amount: u16) {
        match self.state {
            State::Alive { life } if amount >= life => self.state = Dead,
            State::Alive { life } => {
                self.state = Alive {
                    life: life - amount,
                }
            }
            Dead => (),
        }
    }

    fn receive_healing(&mut self) {
        match self.state {
            State::Alive { life: 1000 }  => (),
            State::Alive { .. } => self.state = Alive { life: 1000 },
            Dead => (),
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            level: 1,
            state: Alive { life: 1000 },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Alive { life: u16 },
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

        assert_eq!(Alive { life: 1000 }, character.status());
    }

    #[test]
    fn deals_damage() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 10);

        assert_eq!(Alive { life: 990 }, attackee.status());
    }

    #[test]
    fn dies_when_damage_equals_health() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 1000);

        assert_eq!(Dead, attackee.status());
    }

    #[test]
    fn heals() {
        let attacker = Character::default();
        let healer = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 50);
        healer.heal(&mut attackee);

        assert_eq!(Alive { life: 1000 }, attackee.status());
    }
}
