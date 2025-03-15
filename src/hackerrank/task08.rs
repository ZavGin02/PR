use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let mut max_height = candles[0];
    let mut count = 1;

    for i in 1..candles.len() {
        if candles[i] > max_height {
            max_height = candles[i];
            count = 1;
        } else if candles[i] == max_height {
            count += 1;
        }
    }

    return count; // Add the return statement here
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}