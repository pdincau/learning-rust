#[derive(Copy, Clone)]
pub struct WeightedDamage {
    factor: u16,
}

impl Default for WeightedDamage {
    fn default() -> Self {
        WeightedDamage { factor: 5 }
    }
}

impl WeightedDamage {
    pub fn compute(&self, attacker_level: u16, attackee_level: u16, amount: u16) -> u16 {
        if attackee_level >= self.factor + attacker_level {
            amount / 2
        } else if attacker_level >= self.factor + attackee_level {
            amount * 2
        } else {
            amount
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_attacker_level_is_5_above_attackee_damage_is_doubled() {
        let attacker_level = 10;
        let attackee_level = 5;

        let weighted_damage =
            WeightedDamage::default().compute(attacker_level, attackee_level, 100);

        assert_eq!(200, weighted_damage)
    }

    #[test]
    fn when_attacker_level_is_5_below_attackee_damage_is_decreased_by_half() {
        let attacker_level = 5;
        let attackee_level = 10;

        let weighted_damage =
            WeightedDamage::default().compute(attacker_level, attackee_level, 100);

        assert_eq!(50, weighted_damage)
    }
}
