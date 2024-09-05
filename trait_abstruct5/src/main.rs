// fn generics_return_type() -> Box<dyn std::fmt::Display> {
//     Box::new(1)
// }

fn generics_return_type() -> impl std::fmt::Display {
    1
}

fn main() {
    println!("{}", generics_return_type());
}
