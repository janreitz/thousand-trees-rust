use std::collections::HashMap;

mod l_system;
pub use crate::l_system::produce;

fn main() {
    let axiom = String::from("AB");
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("A"), String::from("AB"));
    production_rules.insert(String::from("B"), String::from("A"));

    let mut production = axiom;
    for _i in 0..10 {
        production = produce(&production, &production_rules);
        println!("{}", production);
    }
}