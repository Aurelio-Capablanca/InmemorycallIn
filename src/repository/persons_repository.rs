use std::ops::Index;
use crate::persons::Person;

pub struct RepositoryPeople {
    persons: Vec<Person>,
}


impl RepositoryPeople {
    pub(crate) fn new() -> Self {
        RepositoryPeople {
            persons: Vec::new(),
        }
    }

    pub(crate) fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }

    pub(crate) fn get_index_position(&self, person: &Person) -> Option<usize> {
        self.persons.iter().position(|p| p == person )
    }

    pub(crate) fn get_by_index(&mut self, index : usize) -> Option<&Person>{
        self.persons.get(index)
    }

    pub(crate) fn get_all_users(&self) {
        for person in &self.persons {
            println!(
                "name: {}, lastname: {}, email: {}",
                person.get_name(),
                person.get_lastname(),
                person.get_email()
            );
        }
    }

}