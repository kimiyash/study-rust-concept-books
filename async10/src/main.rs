async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    println!("{}", ans);
    ans
}

fn main() {
    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    ls.block_on(&rt, async {
        sum_func(10_000_000).await;
        sum_func(20_000_000).await;
    });
}

// 下記のように書くことも可能
// #[tokio::main]
// async fn main() {
//     sum_func(10_000_000).await;
//     sum_func(20_000_000).await;
// }