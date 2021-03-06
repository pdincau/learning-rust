use uuid::Uuid;

use crate::rpg::state::State;
use crate::rpg::state::State::{Alive, Dead};
use crate::rpg::style::Style;
use crate::rpg::style::Style::Melee;
use crate::rpg::weighted_damage::WeightedDamage;

const MAX_LIFE: u16 = 1000;

#[derive(Copy, Clone)]
struct Character {
    id: Uuid,
    level: u16,
    style: Style,
    state: State,
    damage_calculator: WeightedDamage,
}

impl Character {
    pub fn new(style: Style) -> Character {
        Character {
            style,
            ..Character::default()
        }
    }
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

    pub fn attack(self, character: &mut Character, damage: Damage) {
        if self.id == character.id {
            return;
        }
        let attacker_level = self.level;
        let attackee_level = character.level;
        let amount = damage.amount;
        let weighted_damage =
            self.damage_calculator
                .compute(attacker_level, attackee_level, amount);
        character.decrease_life(weighted_damage)
    }

    pub fn heal(&mut self) {
        match self.state {
            State::Alive { life: MAX_LIFE } => (),
            State::Alive { .. } => self.state = Alive { life: MAX_LIFE },
            Dead => (),
        }
    }

    pub fn range(&self) -> u16 {
        self.style.range()
    }

    fn decrease_life(&mut self, amount: u16) {
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

struct Damage {
    pub amount: u16,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: Uuid::new_v4(),
            level: 1,
            style: Melee,
            state: Alive { life: MAX_LIFE },
            damage_calculator: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rpg::state::State::{Alive, Dead};
    use crate::rpg::style::Style::{Melee, Ranged};

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
    fn has_a_max_range_according_to_type() {
        let melee_character = Character::new(Melee);
        let ranged_character = Character::new(Ranged);

        assert_eq!(2, melee_character.range());
        assert_eq!(5, ranged_character.range());
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

        attacker.attack(&mut attackee, Damage { amount: 10 });

        assert_eq!(Alive { life: 990 }, attackee.status());
    }

    #[test]
    fn deals_decreased_damage_if_level_is_5_or_more_levels_below() {
        let attacker = Character::default();
        let mut attackee = Character::default();
        attackee.level = 6;

        attacker.attack(&mut attackee, Damage { amount: 10 });

        assert_eq!(Alive { life: 995 }, attackee.status());
    }

    #[test]
    fn deals_increased_damage_if_level_is_5_or_more_levels_above() {
        let mut attacker = Character::default();
        attacker.level = 6;
        let mut attackee = Character::default();

        attacker.attack(&mut attackee, Damage { amount: 10 });

        assert_eq!(Alive { life: 980 }, attackee.status());
    }

    #[test]
    fn dies_when_damage_equals_health() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.attack(&mut attackee, Damage { amount: 1000 });

        assert_eq!(Dead, attackee.status());
    }

    #[test]
    fn cannot_deal_damage_to_itself() {
        let mut attacker = Character::default();

        attacker.attack(&mut attacker, Damage { amount: 10 });

        assert_eq!(Alive { life: 1000 }, attacker.status());
    }

    #[test]
    fn can_heal_itself() {
        let attacker = Character::default();
        let mut attackee = Character::default();

        attacker.attack(&mut attackee, Damage { amount: 50 });

        attackee.heal();

        assert_eq!(Alive { life: 1000 }, attackee.status());
    }
}
