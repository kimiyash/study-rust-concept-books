// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> Deref for MyBox<T> {
//     type Target = T; // Deref のターゲット型を定義

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox(x);

//     assert_eq!(5, *y); // Deref が自動的に呼び出され、MyBox<T> の中身が得られる
// }

use std::ops::Deref;

struct MyData {
    x: i32,
}

impl Deref for MyData {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

fn deref_data(x: &i32) -> i32 {
    *x
}

fn main() {
    let data = MyData{x: 1};
    let data_deref1 = data.deref();     // &1: &i32型
    let data_deref2 = *(data.deref());   // 1: i32型
    let data_deref3 = *data;             // 1: i32型
    let data_deref4 = deref_data(&data); // 1: i32型

    println!("{}", data_deref1);
    println!("{}", data_deref2);
    println!("{}", data_deref3);
    println!("{}", data_deref4);
    
}
