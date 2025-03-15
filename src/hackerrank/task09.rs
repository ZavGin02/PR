use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let mut parts = s.split(':').collect::<Vec<&str>>();
    let ampm = parts[2].chars().skip(2).collect::<String>();
    let seconds = &parts[2][0..2];
    let mut hours = parts[0].parse::<i32>().unwrap();

    if ampm == "PM" && hours != 12 {
        hours += 12;
    } else if ampm == "AM" && hours == 12 {
        hours = 0;
    }

    format!("{:02}:{:02}:{}", hours, parts[1], seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}