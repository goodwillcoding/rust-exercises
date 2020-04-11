pub fn sum_one_to_n(n: u32) -> u32 {
    // make range from 0 to n + 100, so to include n,
    // then make it an iter and run sum on the iterator value
    let sum: u32 = (0..(n + 1)).into_iter().sum();
    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_0() {
        let result = sum_one_to_n(0);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_1() {
        let result = sum_one_to_n(1);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_sum_100() {
        let result = sum_one_to_n(100);

        assert_eq!(result, 5050);
    }
}
