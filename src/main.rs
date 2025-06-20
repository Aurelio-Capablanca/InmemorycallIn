mod controllers;
mod persons;
mod repository;
mod validations;

use crate::controllers::persons_actions::call_inserts;
use crate::repository::persons_repository::RepositoryPeople;
use crate::validations::validator::validate_integers;
use std::io;
use std::io::Read;

fn main() {
    let mut repository = RepositoryPeople::new();
    let mut input = String::new();
    let mut option: i32 = 0;
    let mut flag: bool = true;
    while flag {
        println!("Choose an option to do: \n1. Create a person row\n2. Update a Person row\n3. Delete a person row\n4. See all the rows\n");
        option = validate_integers(&mut input, "Enter your Choice : ");
        input.clear();
        match option {
            1 => {
                let person = call_inserts(&mut input);
                repository.add_person(person);
            },
            2 => {
                println!("Working !!!");
            },
            4 => {
                repository.get_all_users()
            },
            _ => {
                println!("Exiting !!!");
                flag = false;
            },
        }
        println!("do you want to continue? y/n");
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("n") {
            flag = false;
        }
        input.clear();
    } 
}
