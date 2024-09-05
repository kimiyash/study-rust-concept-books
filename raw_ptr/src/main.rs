use crate::RecursiveEnum::{Val,Null};

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null
}

fn raw_ptr2() {
    let x = Val(Box::new(Val(Box::new(Null))));
    println!("{:?}", x);
}

fn raw_ptr1() {
    let x = 1;
    let x_ptr: *const i32 = &x;

    println!("*x_ptr = {}", unsafe { *x_ptr } );

    let mut y = 2;
    let y_ptr = &mut y as *mut i32;
    unsafe {
        *y_ptr = *x_ptr + 1;
    }
    println!("*y_ptr = {}", unsafe { *y_ptr } );

    println!("y = {}", y)

}

// const と static は大文字で定義することが推奨されている
const BUFSIZE: usize = 1024;
static OFFSET: usize = 15;
fn add_static() {
    const INCREMENT: usize = 2;
    static mut ADD: usize = 1;
    unsafe {
        ADD += INCREMENT;
    }
    println!("add = {}", unsafe { ADD });
}

fn raw_ptr3() {
    let ofset_ref = &OFFSET;
    println!("bufsize = {}", BUFSIZE);
    println!("offset = {}", OFFSET);

    add_static();
    add_static();
    add_static();
}

fn main() {
    raw_ptr1();
    raw_ptr2();
    raw_ptr3();
}
