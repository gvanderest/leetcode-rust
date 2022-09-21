// O(n^2)
pub fn two_sum_original(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    for x in 0..len {
        let val1 = nums.get(x).unwrap();
        for y in 0..len {
            if x == y {
                continue;
            }

            let val2 = *(nums.get(y).unwrap());
            if val1 + val2 == target {
                return Vec::from([x as i32, y as i32]);
            }
        }
    }

    Vec::new()
}

// O(2n) ?.. I'll admit I am terrible at Big-O
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    for x in 0..len {
        let val1 = nums.get(x).unwrap();
        for y in (x + 1)..len {
            if x == y {
                continue;
            }

            let val2 = *(nums.get(y).unwrap());
            if val1 + val2 == target {
                return Vec::from([x as i32, y as i32]);
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
