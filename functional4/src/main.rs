fn fact(n: u32) -> u32 {
    let mut ans = n;
    for ii in (1..=(n-1)).rev() {
        ans += ii;
    }
    ans
}

fn  fact2(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fact2(n - 1)
    }
}

fn main() {
    println!("{}", fact(3));
    println!("---");
    println!("{}", fact2(3));
}
