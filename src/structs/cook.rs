pub struct Cook {
    pub name: String,
    pub specialties: Vec<String>,
}

pub struct CookBuilder {
    name: String,
    specialties: Vec<String>,
}

impl CookBuilder {
    pub fn new(name: &'static str) -> CookBuilder {
        CookBuilder {
            name: name.to_string(),
            specialties: Vec::new(),
        }
    }

    pub fn specialty(&mut self, arg: &'static str) -> &mut CookBuilder {
        self.specialties.push(arg.to_string());
        self
    }

    pub fn build(&self) -> Cook {
        Cook {
            name: self.name.clone(),
            specialties: self.specialties.clone(),
        }
    }
}
