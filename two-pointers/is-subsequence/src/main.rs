fn main() {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut t = t.iter();

        for c in s {
            loop {
                match t.next() {
                    Some(x) => if x == &c { break },
                    None => return false
                }
            }
        }

        return true;
    }

    assert!(is_subsequence(String::from("ac"), String::from("abc")));
}
