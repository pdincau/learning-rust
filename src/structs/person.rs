pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Clone)]
pub struct Person2 {
    pub name: String,
    pub age: u8,
}

pub fn greet(person: &Person) -> () {
    println!("Hello {}", person.name)
}

pub fn greet_without_borrowing(person: Person2) -> () {
    println!("Hello {}", person.name)
}
