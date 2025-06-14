use std::io;

struct Person {
    name: String,
    lastname: String,
    email: String,
}

impl Person {
    fn new() -> Self {
        //println!("Started New Person!!!");
        Person {
            name: String::new(),
            lastname: String::new(),
            email: String::new(),
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_lastname(&mut self, lastname: String) {
        self.lastname = lastname;
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_lastname(&self) -> &str {
        &self.lastname
    }

    fn get_email(&self) -> &str {
        &self.email
    }
}

struct RepositoryPeople {
    persons: Vec<Person>,
}

impl RepositoryPeople {
    fn new() -> Self {
        RepositoryPeople {
            persons: Vec::new(),
        }
    }

    fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }
}

fn call_inserts(input: &mut String) -> Person {
    let mut person = Person::new();
    println!("Enter Your Name ! ");
    io::stdin().read_line(input).unwrap();
    person.set_name(input.trim().to_string());
    input.clear();

    println!("Enter Your Last Name ! ");
    io::stdin().read_line(input).unwrap();
    person.set_lastname(input.trim().to_string());
    input.clear();

    println!("Enter Your Email ! ");
    io::stdin().read_line(input).unwrap();
    person.set_email(input.trim().to_string());
    input.clear();
    person
}

fn main() {
    println!("TESTING!");
    let mut repository = RepositoryPeople::new();
    let mut input = String::new();
    let mut flag: bool = true;
    while flag {
        let person = call_inserts(&mut input); 
        repository.add_person(person);   
        println!("Do you want to continue? y/n");
        io::stdin().read_line(&mut input).unwrap();
        println!("Entered:  {}",input);
        if input.trim().eq_ignore_ascii_case("n") {
            flag = false;
        }
        input.clear();
    }
    
    for person in &repository.persons {
        println!(
            "name: {}, lastname: {}, email: {}",
            person.get_name(),
            person.get_lastname(),
            person.get_email()
        );
    }
}
