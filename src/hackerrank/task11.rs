use std::io::{self, BufRead};

/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 * 1. INTEGER s
 * 2. INTEGER t
 * 3. INTEGER a
 * 4. INTEGER b
 * 5. INTEGER_ARRAY apples
 * 6. INTEGER_ARRAY oranges
 */

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apples_on_house = 0;
    for &apple_dist in apples {
        let landing_pos = a + apple_dist;
        if landing_pos >= s && landing_pos <= t {
            apples_on_house += 1;
        }
    }

    let mut oranges_on_house = 0;
    for &orange_dist in oranges {
        let landing_pos = b + orange_dist;
        if landing_pos >= s && landing_pos <= t {
            oranges_on_house += 1;
        }
    }

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}