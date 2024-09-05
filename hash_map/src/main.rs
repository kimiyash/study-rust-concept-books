use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Japan", "Tokyo");
    capitals.insert("UK", "London");
    capitals.insert("US", "Washington D.C.");

    let targets = vec!["Japan", "US", "France"];
    for tg in targets {
        match capitals.get(tg) {
            Some(s) => println!("The capital of {} is {}", tg, s),
            None => println!("The captal of {} is not found", tg)
        }
    }
}
