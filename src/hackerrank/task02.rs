use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut score = vec![0, 0]; // score[0] - points for Alice, score[1] - points for Bob

    for i in 0..a.len() {
        if a[i] > b[i] {
            score[0] += 1;
        } else if a[i] < b[i] {
            score[1] += 1;
        }
    }

    score
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }
    writeln!(&mut fptr).ok();
}
