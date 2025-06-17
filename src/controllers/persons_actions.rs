use std::io;
use crate::persons::Person;

pub fn call_inserts(input: &mut String) -> Person {
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