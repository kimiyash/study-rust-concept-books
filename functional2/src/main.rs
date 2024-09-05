
fn case1() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = vv.into_iter(); // この時点で vv の所有権が iter に移っている

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    // 以下を呼び出すとすでに vv の所有権が iter になっているのでエラーになる
    // let x = vv[2];
}

fn case2() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = (&vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = vv[2];
    println!("{}", x);
}

fn case3() {
    let mut vv = vec![1, 2, 3, 4];
    let mut iter = (&mut vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    *x += 10;
    println!("{:?}", vv);
}

fn case4() {
    let vv = vec![0, 1, 2, 3, 4];

    for ii in vv {
        println!("{}", ii);
    }

    // 下記は 47 行目の for 文で vv の所有権が iterator に移っているためコンパイルエラーになる
    // for ii in vv {
    //     println!("{}", ii);
    // }
}

fn case5() {
    let vv = vec![0, 1, 2, 3, 4];

    for ii in &(vv) {
        println!("{}", ii);
    }

    for ii in &(vv) {
        println!("{}", ii);
    }
}


fn main() {
    case1();
    println!("---");
    case2();
    println!("---");
    case3();
    println!("---");
    case4();
    println!("---");
    case5();
}
