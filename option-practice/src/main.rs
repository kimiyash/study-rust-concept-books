fn func_ex_div_some(x: i32, y: i32) -> Option<i32>{
    match y {
        0 => None,
        _ => Some(x/y)
    }
}

// fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
//     if let Some(x) = ans {
//         println!("{}", x);
//     } else {
//         println!("None");
//     }
// }
fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if ans.is_some() {
        println!("{}", ans.unwrap());
    } else {
        println!("None");
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None       => println!("None")
    }
}

fn func_ex_unwrap(x: i32) -> Option<i32> {
    if x >= 0 {
        Some(x)
    } else {
        None
    }
}

fn main() {
    func_ex_print_some(func_ex_div_some(9, 4));
    func_ex_print_some(func_ex_div_some(1, 0));

    func_ex_print_some_match(func_ex_div_some(9, 4));
    func_ex_print_some_match(func_ex_div_some(1, 0));

    let rr1 = func_ex_unwrap(3);
    println!("{}", rr1.unwrap());
    println!("{}", rr1.expect("panic panic!!"));

    let rr2 = func_ex_unwrap(-1);
    println!("{}", rr2.unwrap_or(-999));

    // let rr2 = func_ex_unwrap(-1);
    // println!("{}", rr2.expect("panic panic!!"));

    // let rr2 = func_ex_unwrap(-1);
    // println!("{}", rr2.unwrap());

}
