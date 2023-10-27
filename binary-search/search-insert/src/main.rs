fn main() {
    // probably way too lazy but lol
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let result = nums.binary_search(&target);
        match result {
            Ok(x) => return x as i32,
            Err(x) => return x as i32
        }
    }
    
    assert_eq!(search_insert([1,3,5,6].to_vec(), 5), 2);
}
