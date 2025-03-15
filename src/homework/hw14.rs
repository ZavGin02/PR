pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let previous_gray_codes = gray(n - 1);
    let mut result = Vec::new();

    // Add '0' prefix to the previous Gray codes
    for code in &previous_gray_codes {
        result.push(format!("0{}", code));
    }

    // Add '1' prefix to the reversed previous Gray codes
    for code in previous_gray_codes.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data =
            [
                (0, vec!["".to_string()]),
                (1, vec!["0".to_string(), "1".to_string()]),
                (2, vec!["00".to_string(), "01".to_string(), "11".to_string(), "10".to_string()]),
                (3, vec!["000".to_string(), "001".to_string(), "011".to_string(), "010".to_string(),
                         "110".to_string(), "111".to_string(), "101".to_string(), "100".to_string()]),
                (4, vec!["0000".to_string(), "0001".to_string(), "0011".to_string(), "0010".to_string(),
                         "0110".to_string(), "0111".to_string(), "0101".to_string(), "0100".to_string(),
                         "1100".to_string(), "1101".to_string(), "1111".to_string(), "1110".to_string(),
                         "1010".to_string(), "1011".to_string(), "1001".to_string(), "1000".to_string()]),
            ];

        test_data
            .iter()
            .for_each(|(n, out)|
                assert_eq!(gray(*n), *out)
            );
    }
}