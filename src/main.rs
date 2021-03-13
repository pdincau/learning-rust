use crate::structs::cook::CookBuilder;
use structs::developer::Developer;
use structs::person::{greet, greet_without_borrowing, Person, Person2};

mod structs;
mod traits;

fn main() {
    base_struct_and_function();
    base_struct_and_function_without_borrowing();
    more_stuff_on_structs();
    builder_pattern_not_easy();
}

fn builder_pattern_not_easy() {
    let cook = CookBuilder::new("paolo")
        .specialty("pasta")
        .specialty("risotto")
        .build();

    println!("Cook {} loves cooking:", cook.name);
    for dish in cook.specialties.iter() {
        println!("{}", dish)
    }
}

fn more_stuff_on_structs() {
    let mut developer = Developer::new("Paolo", "elixir");
    let _name = developer.name();
    let _favourite_language = developer.favourite_language();
    developer.code();
    developer.learn_language("rust");
    developer.code();

    let past_me = Developer::default();
    past_me.code();
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
