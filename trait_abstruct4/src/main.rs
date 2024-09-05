fn generics_return_type<T: std::fmt::Display>(x: T) -> T {
    x
}

fn main() {
    println!("{}", generics_return_type(1));
    println!("{}", generics_return_type("Hello"));
}
