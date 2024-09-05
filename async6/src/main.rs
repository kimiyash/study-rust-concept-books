use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let (tx, rx) = channel();

    let data_len = data.len();

    for dd in data {
        let x = tx.clone();
        spawn(move || x.send(dd));
    }

    for _ in 0..data_len {
        println!("{}", rx.recv().unwrap()); // rx.recv() で join と同様にブロックされる
    }
}
