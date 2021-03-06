pub struct Developer {
    name: String,
    favourite_language: String,
}

impl Developer {
    pub fn new(name: &'static str, favourite_language: &'static str) -> Self {
        Self {
            name: name.to_string(),
            favourite_language: favourite_language.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn favourite_language(&self) -> &String {
        &self.favourite_language
    }

    pub fn code(&self) -> () {
        println!("I write code in: {}", self.favourite_language);
    }

    pub fn learn_language(&mut self, language: &'static str) -> () {
        self.favourite_language = language.to_string();
    }
}

impl Default for Developer {
    // this conflicts with #[derive(Default)]
    fn default() -> Self {
        Developer::new("Paolo", "erlang")
    }
}
