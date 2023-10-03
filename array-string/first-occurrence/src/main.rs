fn main() {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = needle.as_str();

        match haystack.find(needle) {
            Some(x) => { return x as i32 },
            None => return -1
        }
    }

    assert_eq!(str_str(String::from("sadbutsad"), String::from("sad")), 0);
    assert_eq!(str_str(String::from("leetcode"), String::from("leeto")), -1);
}
