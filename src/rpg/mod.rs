use crate::rpg::State::{Alive, Dead};
use uuid::Uuid;

const MAX_LIFE: u16 = 1000;

#[derive(Copy, Clone)]
struct Character {
    id: Uuid,
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

    pub fn deal_damage(self, character: &mut Character, damage: u16) {
        if self.id == character.id {
            return;
        }
        let weighted_damage = weight_damage(self.level, character.level, damage);
        character.receive_damage(weighted_damage)
    }

    pub fn heal(&mut self) {
        match self.state {
            State::Alive { life: MAX_LIFE } => (),
            State::Alive { .. } => self.state = Alive { life: MAX_LIFE },
            Dead => (),
        }
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
}

fn weight_damage(attacker_level: u16, attackee_level: u16, amount: u16) -> u16 {
    if attackee_level >= 5 * attacker_level {
        amount / 2
    } else {
        amount
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: Uuid::new_v4(),
            level: 1,
            state: Alive { life: MAX_LIFE },
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
    fn deals_decreased_damage_if_level_is_5_or_more_levels_above() {
        let attacker = Character::default();
        let mut attackee = Character::default();
        attackee.level = 6;

        attacker.deal_damage(&mut attackee, 10);

        assert_eq!(Alive { life: 995 }, attackee.status());
    }

    #[test]
    fn dies_when_damage_equals_health() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 1000);

        assert_eq!(Dead, attackee.status());
    }

    #[test]
    fn cannot_deal_damage_to_itself() {
        let mut attacker = Character::default();

        attacker.deal_damage(&mut attacker, 10);

        assert_eq!(Alive { life: 1000 }, attacker.status());
    }

    #[test]
    fn can_heal_itself() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.deal_damage(&mut attackee, 50);
        attackee.heal();

        assert_eq!(Alive { life: 1000 }, attackee.status());
    }
}
