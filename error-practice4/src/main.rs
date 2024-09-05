use std::io::BufReader;
use std::io::BufRead;
use thiserror::Error;
use std::fs::File;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("./input.txt")?;
    let f = BufReader::new(f);

    for line in f.lines().flatten() {
        let mut v = Vec::new();
        for ee in line.split(' ') {
            v.push(ee.parse()?);
        }
        let result = mydiv(v[0], v[1])?;
        println!("{} / {} = {}", v[0], v[1], result);
    }
    Ok(())
}
