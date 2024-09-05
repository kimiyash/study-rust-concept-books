use thiserror::Error;

#[derive(Error, Debug)]
enum DivError{
    #[error("{0} devied by zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are nagative")]
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

// fn print_mydiv(x: i32, y: i32) {
//     match mydiv(x, y) {
//         Ok(ans) => println!("no error. ans = {}", ans),
//         Err(e) => println!("{}", e)
//     }
// }
fn print_mydiv(x: i32, y: i32) {
    let ret = mydiv(x, y);
    if ret.is_ok() {
        println!("no error. ans = {}", ret.unwrap())
    } else {
        println!("{}", ret.err().unwrap())
    }

}

fn main() {
    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5,-2);
}
