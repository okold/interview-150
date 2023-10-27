fn main() {
    // messy, probably a better way to do this
    pub fn my_sqrt(x: i32) -> i32 {
        let mut guess = x / 2;
        let mut undershot = false;

        // 46340 = sqrt(2^31), prevents overflow
        if guess > 46339 {
            guess = 46339;
        }

        loop {
            if guess > 46340 {
                return 46340;
            }

            let square = guess * guess;
    
            if square == x { return guess };

            if square > x && undershot {
                return guess - 1;
            }

            if square < x {
                guess += 1; // could probably do a binary search instead of adding 1 at this point
                undershot = true;
            }
            else {
                guess /= 2;
                undershot = false;
            }
        }
    }

    assert_eq!(my_sqrt(2), 1);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(2147395599), 46339);
    assert_eq!(my_sqrt(2147483647), 46340);
}
