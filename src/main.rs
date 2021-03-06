use crate::structs::{greet, Person};

mod structs;

fn main() {
    base_struct_and_function();
}

fn base_struct_and_function() {
    let me = Person {
        name: "Paolo".to_string(),
        age: 36,
    };
    greet(&me);
    greet(&me);
}
