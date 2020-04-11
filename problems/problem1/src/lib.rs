pub fn find_largest_element(elems: &Vec<i64>) -> Option<i64> {
    if elems.is_empty() {
        return None;
    } else {
        let mut biggest = &elems[0];
        for contender in elems[1..].iter() {
            if contender > biggest {
                biggest = contender
            }
        }
        return Some(*biggest);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_elements() {
        let empty_vec = Vec::new();

        let result = find_largest_element(&empty_vec);

        assert_eq!(result, None);
    }

    #[test]
    fn test_some_elements() {
        let myvec = vec![1, 5, 3, 8, 0];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_negative_elements() {
        let myvec = vec![-1, -5, -3, -8];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(-1));
    }
}
