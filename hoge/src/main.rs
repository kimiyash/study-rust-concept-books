
fn hoge(x: i32) -> i32 {
    x + 1024
}

fn uhi() -> i64 {
    65535
}

fn main() {
    println!("Hello, world!");
    println!("{}", hoge(32));
    println!("{}", uhi())
}

