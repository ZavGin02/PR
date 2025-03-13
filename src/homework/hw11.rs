use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> Option<((usize, usize), i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_indexes = (0, 1);

    for i in 1..data.len() - 1 {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_indexes = (i, i + 1);
        }
    }

    Some((min_indexes, min_sum))
}

pub fn print_min_adjacent_sum(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{}. ", i)).collect::<String>());
    println!("data: {:?}", data);

    if let Some(((idx1, idx2), sum)) = min_adjacent_sum(data) {
        let mut indexes_str = String::new();
        for i in 0..data.len() {
            if i == idx1 {
                indexes_str.push_str("\\__");
            } else if i == idx2 {
                indexes_str.push_str("__/");
            } else {
                indexes_str.push_str("    ");
            }
        }
        println!("indexes: {}", indexes_str);
        println!("min adjacent sum={} + {} = {} at indexes: {}, {}", data[idx1], data[idx2], sum, idx1, idx2);
    } else {
        println!("Недостатньо елементів для знаходження суми.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_random_vector() {
        let vec = gen_random_vector(20);
        assert_eq!(vec.len(), 20);
        for &val in vec.iter() {
            assert!(val >= 10 && val < 100);
        }
    }

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22];
        let result = min_adjacent_sum(&data).unwrap();
        assert_eq!(result.0, (5, 6));
        assert_eq!(result.1, 82);
    }
}