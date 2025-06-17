mod persons;
mod repository;
mod controllers;

use crate::repository::persons_repository::RepositoryPeople;
use crate::controllers::persons_actions::call_inserts;
use std::io;


fn main() {
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
