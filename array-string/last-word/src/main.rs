fn main() {
    pub fn length_of_last_word(s: String) -> i32 {
        let s: Vec<&str> = s.trim().split(' ').collect();

        match s.iter().last() {
            Some(x) => return x.len() as i32,
            None => return 0
        }
    }

    assert_eq!(length_of_last_word(String::from("abc dsfsg sdf   ")), 3);
}
