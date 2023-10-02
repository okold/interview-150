fn main() {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let target: usize = (nums.len() + 2 - 1) / 2;

        for n in nums.iter() {
            if nums.iter().filter(|&x| *x == *n).count() >= target {
                return *n;
            }
        }

        return -1;
    }

    assert_eq!(majority_element(vec![1,3,3]), 3);
}
