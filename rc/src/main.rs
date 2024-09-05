use std::rc::Rc;

struct TestA {
    data_i32: i32,
    data_string: String,
}

fn main() {
    let x = Rc::new(TestA {
        data_i32: 1,
        data_string: String::from("Hello"),
    });
    let data_i32 = &x.data_i32;
    let data_string = &x.data_string;
    println!("{}, {}", data_i32, data_string);
}
