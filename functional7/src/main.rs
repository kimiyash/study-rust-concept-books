fn func_of_func(b :i32) -> impl Fn(i32) -> i32 {
    move |x| x + b // move は環境の変数をクロージャ内に移動するためのキーワード
}

fn main() {
    let add_2 = func_of_func(2);
    println!("{}", add_2(1));
    println!("{}", add_2(4));
}
