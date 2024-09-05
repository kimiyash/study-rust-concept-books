use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut lines_vec = Vec::new();

    for ll in f.lines() {
        lines_vec.push(ll.unwrap());
    }

    println!("{:?}", lines_vec);
    Ok(())
}