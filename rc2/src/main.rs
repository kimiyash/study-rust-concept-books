use std::rc::Rc;

struct DataA {
    number_a: Option<Rc<i32>>,
}

struct DataB {
    number_b: Option<Rc<i32>>,
}

fn setdata(data_a: &mut DataA, data_b: &mut DataB, value: i32) {
    let number = Rc::new(value + 10);
    data_a.number_a = Some(Rc::clone(&number));
    data_b.number_b = Some(Rc::clone(&number));
}

fn main() {
    let mut data_a_1 = DataA { number_a: None };
    let mut data_b_1 = DataB { number_b: None };

    let mut data_a_2 = DataA { number_a: None };
    let mut data_b_2 = DataB { number_b: None };

    setdata(&mut data_a_1, &mut data_b_1, 1);
    setdata(&mut data_a_2, &mut data_b_2, 2);

    println!("to be 11, 11: {} {}", data_a_1.number_a.unwrap(), data_b_1.number_b.unwrap());
    println!("to be 12, 12: {} {}", data_a_2.number_a.unwrap(), data_b_2.number_b.unwrap());
}
