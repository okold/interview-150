fn main() {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut offset: usize = 0; // offset to shift values by
        let mut i: usize = 0;      // initial index

        while i < nums.len() {
            // set the current value and move it to the
            // correct position
            let curr: i32  = nums[i];
            nums[i-offset] = curr;

            let mut dupes = false; // flag to ignore after second
            let mut j = i + 1;  // index of duplicate checker

            while j < nums.len() {
                if nums[j] == curr {
                    if !dupes {     // second value, keep and move
                        dupes = true;
                        nums[j-offset] = curr;
                    }
                    else {  // 3rd+ value, ignore and increase offset
                        offset += 1;
                    }
                }
                else {  // non-duplicate
                    break;
                }
                j += 1;
            }
            i = j;
        }

        return (nums.len()-offset) as i32;
    }

    assert_eq!(7, remove_duplicates(& mut [1,1,1,2,2,3,3,3,5].to_vec()));
}
