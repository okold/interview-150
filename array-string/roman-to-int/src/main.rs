//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000

fn main() {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut symbols = HashMap::new();
        symbols.insert('I', 1);
        symbols.insert('V', 5);
        symbols.insert('X', 10);
        symbols.insert('L', 50);
        symbols.insert('C', 100);
        symbols.insert('D', 500);
        symbols.insert('M', 1000);

        let mut val = 0;
        let mut prev = 9999;
        let mut add_last = true;
        for c in s.chars(){
            //first case
            if prev == 9999 {
                prev = symbols.get(&c).copied().unwrap_or(0);
            }
            else {
                let curr = symbols.get(&c).copied().unwrap_or(0);
                // prev < curr, ie IV case
                if prev < curr {
                    val += curr - prev;
                    add_last = false;
                }
                // avoids extra adds
                else if add_last == true {
                    val += prev;
                    add_last = true;
                }
                else {
                    add_last = true;
                }
                prev = curr;
            }
        }
        // handles single-digit or odd cases, ie I or VII
        if add_last {
            val += prev;
        }

        return val;
    }

    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    assert_eq!(roman_to_int(String::from("MCMXCVI")), 1996);
}
