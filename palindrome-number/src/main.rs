fn main() {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x < 10 {
            return true;
        }

        let mut num_digits: i32 = 0;
        let mut x_mut: i32 = x;

        while x_mut != 0 {
            x_mut /= 10;
            num_digits += 1;
        }

        //println!("{}, {}", x, num_digits);

        x_mut = x;
        let mut rev: i32 = 0;
        let mut counter: i32 = num_digits-1;
        let base: i32 = 10;

        for _ in 0..num_digits {
            //println!("{}, {}, 10^{}", x_mut % 10, (x_mut % 10) * base.pow(counter as u32), counter);
            rev += (x_mut % 10) * base.pow(counter as u32);
            counter -= 1;
            x_mut = x_mut / 10;
        }

        //println!("rev: {}", rev);

        if rev == x {
            return true;
        }

        return false;
    }

    assert_eq!(is_palindrome(-100), false);
    assert_eq!(is_palindrome(121), true);
}
