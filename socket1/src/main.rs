use std::io::{self, BufReader};
use std::net::TcpListener;

mod line_read;

// ターミナルで `echo "test" | nc 127.0.0.1 3210` で動作確認できる
fn main() -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:3210")?;

    for stream in listner.incoming() {
        let lines_vec = line_read::get_lines(stream?);
        println!("{:?}", lines_vec);
    }
    Ok(())
}
