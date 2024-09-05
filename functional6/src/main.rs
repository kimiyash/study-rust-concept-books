fn main() {
    let v = vec![0, 1, 2, 3];
    let filter: Vec<i32> = v.into_iter().filter(|&x| x >= 2).collect();
    println!("{:?}", filter);

    let w = vec![4, 5, 6];
    let sum = w.into_iter().fold(0, |x, y| x + y);
    println!("{}", sum);

    let w = vec![4, 5, 6];
    let sum = w.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", sum);
}
