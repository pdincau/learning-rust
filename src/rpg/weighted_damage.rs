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
        if attackee_level >= self.factor * attacker_level {
            amount / 2
        } else if attacker_level >= self.factor * attackee_level {
            amount * 2
        } else {
            amount
        }
    }
}
