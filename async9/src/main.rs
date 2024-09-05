async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    println!("{}", ans);
    ans
}

fn main() {
    let fut = sum_func(10_000_000);

    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    ls.block_on(&rt, fut);
}
 
