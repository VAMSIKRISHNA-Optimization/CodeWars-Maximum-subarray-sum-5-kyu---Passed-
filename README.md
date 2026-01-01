# CodeWars-Maximum-subarray-sum-5-kyu---Passed-
The maximum sum subarray problem consists in finding the maximum sum of a contiguous subsequence in an array or list of integers:

For example:

Input: [-2, 1, -3, 4, -1, 2, 1, -5, 4]
Output: 6 (Sum of [4, -1, 2, 1])
Easy case is when the list is made up of only positive numbers and the maximum sum is the sum of the whole array. If the list is made up of only negative numbers, return 0 instead. Your solution should be fast, it will be tested on very large arrays so slow solutions will time out.

Empty list is considered to have zero greatest sum. Note that the empty list or array is also a valid sublist/subarray.



#[cfg(test)]
mod tests {
    use super::max_sequence;
    use std::cmp::max;

    #[test]
    fn fixed_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
        assert_eq!(max_sequence(&Vec::new()), 0);
        assert_eq!(max_sequence(&[-2, 1, -7, 4, -10, 2, 1, 5, 4]), 12);
        assert_eq!(max_sequence(&[-83, 56, 61, -12, 39, -68, 44, -40, -47, -93]), 144);
        assert_eq!(max_sequence(&[-32, -24, -43, -43, -55, 75, -24, 55, -49, 37]), 106);
        assert_eq!(max_sequence(&[58, 26, 51, -88, 25, -27, -45, 86, 32, 48]), 166);
    }

    fn reference_solution(seq: &[i32]) -> i32 {
        let (mut m, mut c): (i32, i32) = (0, 0);
        for n in seq {
            c = max(0, c + n);
            m = max(c, m);
        }
        m
    }

    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let len = rng.gen_range(0..51);
            let input: Vec<i32> = (0..len).map(|_| rng.gen_range(-100..101)).collect();
            let expected = reference_solution(&input);
            assert_eq!(max_sequence(&input), expected, "failed on input: {:#?}", input);
        }
    }
}
