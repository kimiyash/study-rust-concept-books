enum DivError {
    DivByZero(i32),
    BothNegative(i32, i32),
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x :i32, y: i32) {
    match mydiv(x, y) {
        Ok(ans) => println!("no error. ans {}", ans),
        Err(DivError::DivByZero(a)) => {
            println!("{} div by zero", a)
        },
        Err(DivError::BothNegative(a, b)) => {
            println!("Both numerator {} and denominator {} are negative)", a, b)
        }
    }
}

fn main() {
    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5,-2);
}
