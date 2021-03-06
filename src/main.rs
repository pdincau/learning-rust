use crate::structs::{Person, greet};

mod structs;

fn main() {
    let me = Person { name: "Paolo".to_string(), age: 36 };
    greet(me);
}
