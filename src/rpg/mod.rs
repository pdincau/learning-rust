struct Character {
    health: u32
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn health_starts_at_1000() {
        let character = Character { health: 1000 };
        assert_eq!(1000, character.health);
    }
}
