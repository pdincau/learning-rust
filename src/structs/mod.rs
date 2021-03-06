
pub struct Person {
    pub name: String,
    pub age: u8
}

pub fn greet(person: Person) -> () {
    println!("Hello {}", person.name)
}