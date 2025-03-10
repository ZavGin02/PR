use std::io;

pub fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let _n: i32 = n.trim().parse().expect("Please type a number!");

    let mut ar_input = String::new();
    io::stdin().read_line(&mut ar_input).expect("Failed to read line");

    let ar: Vec<i32> = ar_input
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let result = simple_array_sum(&ar);
    println!("{}", result);
}

#[test]
fn test_simple_array_sum() {
    assert_eq!(simple_array_sum(&[1, 2, 3, 4, 10, 11]), 31);
    assert_eq!(simple_array_sum(&[1, 2, 3]), 6);
    assert_eq!(simple_array_sum(&[-1, -2, -3]), -6);
    assert_eq!(simple_array_sum(&[0]), 0);
    assert_eq!(simple_array_sum(&[]), 0);
}