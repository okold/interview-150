fn main() {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count: usize = 0;
        let mut prev: i32 = 0;
        let length: usize = nums.len();

        for i in 0..length {
            if i == 0 {
                prev = nums[i];
            }
            else if nums[i] == prev {
                count += 1;
            }
            else {
                prev = nums[i];
                nums[i - count] = nums[i];
            }
        }

        return length as i32 - count as i32;
    }

    let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];

    assert_eq!(remove_duplicates(&mut nums), 5);
}
