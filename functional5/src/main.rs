fn add_one_to_vec(vv: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    for ee in vv {
        ret.push(ee + 1);
    }
    ret
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let vv = vec![0, 1, 2, 3];
    println!("{:?}", add_one_to_vec(vv));

    let vv: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
    println!("{:?}", vv);

    let mut vv = vec![0, 1, 2, 3];
    vv = vv.into_iter().map(add_one).collect();
    println!("{:?}", vv);

    let vv: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(|x| x + 1).collect();
    println!("{:?}", vv);

    let vv: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(|x: i32|->i32 { x + 1 }).collect();
    println!("{:?}", vv);

    let add_one = |x| x + 1;
    let vv: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
    println!("{:?}", vv);

    // 下記はクロージャのなかみで m を借用しているため動作しない
    // let mut m = 1;
    // let add_m = |x| x + m;
    // println!("{}", add_m(1));
    // m = 10;
    // println!("{}", add_m(1));

    // 下記の場合二回目の let m は一度目の let m を覆い隠しているだけなので初回の m がクロージャに束縛されるので
    // 出力は 2 が２回表示される
    let m = 1;
    let add_m = |x| x + m;
    println!("{}", m);

    let m = 10;
    println!("{}", add_m(1));

    // ちゃんと動作させたい場合はクロージャを宣言しなおす必要がある
    let m = 1;
    let add_m = |x| x + m;
    println!("{}", add_m(1));

    let m = 10;
    let add_m = |x| x + m;
    println!("{}", add_m(1));
}
