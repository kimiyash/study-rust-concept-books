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

fn repeat_mydiv(ary: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();
    for aa in ary {
        let ans = mydiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans);
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e)
    }
}
fn main() -> Result<(), DivError> {
    println!("1st calc");
    print_repeat_mydiv(repeat_mydiv(&[(2 ,1), (9, 3)]));
    println!();

    println!("2nd calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, -3), (5, 2)]));
    println!();

    println!("3rd calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, 0), (6, 3)]));
    println!();

    println!("{}", mydiv(3, 2)?);

//    println!("{}", mydiv(3, 0)?);
    Ok(())
}
