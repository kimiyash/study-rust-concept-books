use std::cmp::Ordering;

enum Sign {
    Positive,
    Zero,
    Negative
}

fn determine_sign(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Equal   => Sign::Zero,
        Ordering::Less    => Sign::Negative
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Negative => println!("-"),
        Sign::Zero     => println!("0")
    }
}

fn main() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}
