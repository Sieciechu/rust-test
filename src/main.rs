#[macro_use]
extern crate uuid;

mod sound;

mod performance_group {
    use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

mod plant {

    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String,
        id: uuid::Uuid,
    }

    impl Vegetable {
        pub fn new(name: String) -> Vegetable {
            Vegetable {
                name,
                id: uuid::Uuid::new_v4(),
            }
        }

        pub fn get_id(self : &Vegetable) -> String {
            self.id.to_string()
        }
    }
}

use std::collections::HashMap;
use std::io;
use std::fmt;

fn main() {

    crate::sound::instrument::clarinet();
    sound::instrument::clarinet();

    let tomato = plant::Vegetable::new(String::from("tomato"));
    eprintln!("tomato = {:#?}", tomato.get_id());
    eprintln!("tomato.name = {:?}", tomato.name);

    performance_group::clarinet_trio();

    let map: HashMap<String, i32> = HashMap::new();

}
