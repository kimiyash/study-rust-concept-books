fn func_of_func(b: i32) -> impl FnMut(i32) -> i32 {
    let mut count = 0;
    move |x| {
        count += 1;
        println!("count: {}", count);
        x + b
    }
}

fn main() {
    let mut add2 = func_of_func(2);
    println!("{}", add2(1));
    println!("{}", add2(2));

    let mut add3 = func_of_func(3);
    println!("{}", add3(1));
    println!("{}", add3(2));
}
