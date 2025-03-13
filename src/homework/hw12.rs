pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    let average = total / n as u32;

    let mut moves = 0;
    let mut balance: Vec<i32> = shipments
        .iter()
        .map(|&x| x as i32 - average as i32)
        .collect();

    for i in 0..n - 1 {
        if balance[i] != 0 {
            balance[i + 1] += balance[i];
            moves += 1;
        }
    }

    moves
}