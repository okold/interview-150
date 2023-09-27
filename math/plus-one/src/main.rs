fn main() {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {

        let mut plus_one = digits.clone();

        for (i, digit) in digits.iter().enumerate().rev() {
            if digit < &9 {
                plus_one[i] += 1;
                break;
            }
            else {
                plus_one[i] = 0;
                
                if i == 0 {
                    let mut hold = plus_one.clone();
                    plus_one = [1].to_vec();
                    plus_one.append(&mut hold);
                }
            }
            
        }

        return plus_one;
    }

    assert_eq!(plus_one([1,2,3].to_vec()), [1,2,4].to_vec());
    assert_eq!(plus_one([4,3,2,1].to_vec()), [4,3,2,2].to_vec());
    assert_eq!(plus_one([9].to_vec()), [1,0].to_vec());
}
