pub fn jump(nums: Vec<i32>) -> i32 {
    let reversed_list: Vec<i32> = nums.into_iter().rev().collect();

    0
}

#[cfg(test)]
mod jump_tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![1, 2];
        let result = jump(input);
        let expected_result = 1;
        assert_eq!(result, expected_result);
    }
}
