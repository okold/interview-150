fn main() {
    // pretty slow, would probably be faster with slices
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let val = nums.pop();
            match val {
                Some(x) => nums.insert(0, x),
                None => ()
            }
        }
    }

    rotate(& mut [1,2,3,4].to_vec(), 2);
}
