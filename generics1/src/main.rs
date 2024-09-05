
fn return_input<T>(x: T) -> T
{
    x
}


fn main() {
    let x1 = return_input(1);
    let x2 = return_input(String::from("hello"));
    let x3 = return_input::<f64>(2.0);

    println!("{}", x1);
    println!("{}", x2);
    println!("{}", x3);
}
