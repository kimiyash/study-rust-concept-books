use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;

const BUFSIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let mut fr = File::open("input.txt")?;
    let mut fw = File::create("output.txt")?; // file を作成して書き込む
    // let OpenOptions::new().read(true).write(true).append(true).open("output.txt"); // file を追記モードで書き込むとき

    let mut buf =   [0_u8; BUFSIZE];
    
    loop {
        let read_size = fr.read(&mut buf)?;

        if read_size == 0 {
            break;
        } else {
            let _ = fw.write(&buf[..read_size])?;
        }
    }
    Ok(())
}

// file の書き込みモード指定追記モードでファイルを開いて書き込むとき
// use std:fs::OpenOptions;
//
// let file = OpenOptions::new()
//  .read(true)
//  .wirte(true)
//  .append(true)
//  .open("file.txt")
