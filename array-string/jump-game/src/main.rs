fn main() {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        fn can_jump_rec(nums: & Vec<i32>, cheat_sheet: & mut Vec<Option<bool>>, current: usize, max_index: usize) -> bool {

            // base cases
            if max_index == 0 || current >= max_index {
                return true;
            }
            else if nums[current] == 0 {
                let _ = cheat_sheet[current].insert(false);
                return false;
            }
            
            // cheat sheet short-circuits pre-calculated results
            match cheat_sheet[current] {
                Some(x) => {
                    if x == false {
                        return false;
                    }
                    else {
                        return true;
                    }
                },
                None => {
                    // attempt all possible jumps from current point
                    let mut val = nums[current];
                    while val > 0 {
                        if !can_jump_rec(nums, cheat_sheet, val as usize + current, max_index) {
                            let _ = cheat_sheet[current].insert(false);
                            val -= 1;
                        }
                        else {
                            let _ = cheat_sheet[current].insert(true);
                            return true;
                        }
                    }
                }
            }
            let _ = cheat_sheet[current].insert(false);
            return false;
        }

        let len = nums.len();
        let mut cheat_sheet: Vec<Option<bool>> = vec![None; len];
        return can_jump_rec(&nums, & mut cheat_sheet, 0, len-1);
    }

    assert_eq!(true, can_jump([2,3,1,1,4].to_vec()));
    assert_eq!(false, can_jump([3,2,1,0,4].to_vec()));
    assert_eq!(false, can_jump([2,0,6,9,8,4,5,0,8,9,1,2,9,6,8,8,0,6,3,1,2,2,1,2,6,5,3,1,2,2,6,4,2,4,3,0,0,0,3,8,2,4,0,1,2,0,1,4,6,5,8,0,7,9,3,4,6,6,5,8,9,3,4,3,7,0,4,9,0,9,8,4,3,0,7,7,1,9,1,9,4,9,0,1,9,5,7,7,1,5,8,2,8,2,6,8,2,2,7,5,1,7,9,6].to_vec()));
}
