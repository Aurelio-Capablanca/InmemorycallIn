mod persons;
mod repository;
use crate::repository::persons_repository::RepositoryPeople;
use crate::persons::Person;
use std::io;




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
        println!("do you want to continue? y/n");
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("n") {
            flag = false;
        }
        input.clear();
    }
    repository.get_all_users();
}
