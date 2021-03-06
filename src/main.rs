use crate::structs::{greet, greet_without_borrowing, Developer, Person, Person2};

mod structs;

fn main() {
    base_struct_and_function();
    base_struct_and_function_without_borrowing();
    more_stuff_on_structs();
}

fn more_stuff_on_structs() {
    let mut developer = Developer::new("Paolo", "elixir");
    let _name = developer.name();
    let _favourite_language = developer.favourite_language();
    developer.code();
    developer.learn_language("rust");
    developer.code();
}

fn base_struct_and_function() {
    let me = Person {
        name: "Paolo".to_string(),
        age: 36,
    };
    greet(&me);
    greet(&me);
}

fn base_struct_and_function_without_borrowing() {
    let me = Person2 {
        name: "Paolo".to_string(),
        age: 36,
    };
    greet_without_borrowing(me.clone());
    greet_without_borrowing(me);
}
