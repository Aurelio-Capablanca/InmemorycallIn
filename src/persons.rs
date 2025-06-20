#[derive(PartialEq)]
pub struct Person {
    name: String,
    lastname: String,
    email: String,
}

impl Person {
    pub fn new() -> Self {
        Person {
            name: String::new(),
            lastname: String::new(),
            email: String::new(),
        }
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn set_lastname(&mut self, lastname: String) {
        self.lastname = lastname;
    }

    pub(crate) fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_lastname(&self) -> &str {
        &self.lastname
    }

    pub(crate) fn get_email(&self) -> &str {
        &self.email
    }
}