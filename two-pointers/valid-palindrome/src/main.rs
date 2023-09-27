fn main() {
    pub fn is_palindrome(s: String) -> bool {

        let clean = s.replace(|c: char| !c.is_alphanumeric(), "").to_lowercase();
        let half: usize = clean.len()/2;

        let mut worker = clean.clone();
        let mut palindrome = true;
        let mut count: usize = 0;

        for c in clean.chars() {
            if count > half {
                break;
            }

            let last = worker.pop();

            match last {
                Some(x) => if c != x {
                    palindrome = false;
                    break;
                },
                None => (),
            }
            count += 1;
        }

        return palindrome;

    }


    assert_eq!(is_palindrome(String::from("aa")), true);
    assert_eq!(is_palindrome(String::from("aab")), false);
    assert_eq!(is_palindrome(String::from("a")), true);
}
