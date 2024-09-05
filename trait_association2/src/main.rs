use std::ops::Neg;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;

trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output
    where
        Self: Sized + PartialOrd + Neg + From<i8> + TryInto<<Self as IAbs>::Output>,
        <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>,
        <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
        <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug
        {
            if self >= (0_i8).into() {
                self.try_into().unwrap()
            } else {
                (-self).try_into().unwrap()
            }
        }
}

impl IAbs for i8 {
    type Output = u8;
}

impl IAbs for i16 {
    type Output = u16;
}

impl IAbs for i32 {
    type Output = u32;
}

impl IAbs for i64 {
    type Output = u16;
}

fn main() {
    println!("{:?}", (2_i32).abs());
    println!("{:?}", (2_i8).abs());
    println!("{:?}", (2_i16).abs());
    println!("{:?}", (2_i64).abs());

    println!("{:?}", (-2_i32).abs());
    println!("{:?}", (-2_i8).abs());
    println!("{:?}", (-2_i16).abs());
    println!("{:?}", (-2_i64).abs());
}
