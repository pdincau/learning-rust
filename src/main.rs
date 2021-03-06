use crate::structs::{greet, Person};

mod structs;

fn main() {
    let me = Person {
        name: "Paolo".to_string(),
        age: 36,
    };
    greet(&me);
    greet(&me);
}
