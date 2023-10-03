

fn main() {
    /* keeping this solution as an example, problem is it hits overflow errors, even with i64 
    pub fn add_binary(a: String, b: String) -> String {
        let a = i32::from_str_radix(&a, 2).expect("not binary");
        let b = i32::from_str_radix(&b, 2).expect("not binary");

        return format!("{:b}", a + b);
    }
    */

    pub fn add_binary(a: String, b: String) -> String {
        let max;
        let min;
        if a.len() > b.len() {
            max = a;
            min = b;
        }
        else {
            min = a;
            max = b;
        }

        let min: Vec<i32> = min.chars().map(|c| if c == '1' {1} else {0}).rev().collect();
        let min_len = min.len();

        let mut max: Vec<i32> = max.chars().map(|c| if c == '1' {1} else {0}).rev().collect();
        let mut carry = false;

        for i in 0..max.len() {
            if carry == true {
                max[i] += 1;
                carry = false;
            }

            if i < min_len {
                max[i] += min[i];
            }

            if max[i] == 2 {
                max[i] = 0;
                carry = true;
            }
            else if max[i] == 3 {
                max[i] = 1;
                carry = true;
            }
        }

        if carry == true {
            max.push(1);
        }

        let mut ans = String::new();

        for x in max.iter().rev() {
            ans += &x.to_string();
        }

        return ans;
    }


    assert_eq!(String::from("100"), add_binary(String::from("11"), String::from("1")));
}
