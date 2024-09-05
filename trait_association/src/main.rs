trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output;
}

impl IAbs for i32 {
    type Output = u32;
    fn iabs(self) -> <Self as IAbs>::Output {
        if self >= 0 {
            self as <Self as IAbs>::Output
        } else {
            (-self) as <Self as IAbs>::Output
        }
    }
}

fn main() {
    println!("{}", 1.iabs());
    println!("{}", (-1).iabs());
}
