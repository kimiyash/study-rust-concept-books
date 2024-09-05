
fn case1() {
    let c = "あいうえお";
    // let c_vec = c.chars().collect::<Vec<char>>();
    // 上記は推論によって下記に書き換えられる _ はコンパイラに推論を求める
    // let c_vec = c.chars().collect::<Vec<_>>();
    // さらに下記でも動作する
    let c_vec: Vec<_> = c.chars().collect();
    println!("{:?}", c_vec);
}

// collect を使わない場合
fn case2() {
    let c = "あいうえお";
    let mut c_vec = Vec::new();
    for cc in c.chars() {
        c_vec.push(cc);
    }
    println!("{:?}", c_vec);
}

fn main() {
    case1();
    println!("---");
    case2();
    println!("---");
}
