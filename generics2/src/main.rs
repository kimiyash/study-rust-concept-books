struct GenEx<T> {
    value: T,
}

impl<T> GenEx<T> {
    fn return_value(self) -> T {
        self.value
    }
}

fn main() {
    let x1 = GenEx{value: 1};
    let x2 = GenEx{value: String::from("hello")};
    let x3 = GenEx::<f64>{value: 2.0};

    println!("{}", x1.return_value());
    println!("{}", x2.return_value());
    println!("{}", x3.return_value());
}
