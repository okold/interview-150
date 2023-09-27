fn main() {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let ransom_note = ransom_note.chars();
        let mut magazine = magazine;

        for c in ransom_note {
            if magazine.contains(c) {
                // split string by c, re-insert c into all splits other than first
                let mut worker = String::new();
                let parts = magazine.split(c);
                let mut count = 0;
                for p in parts {
                    if count <= 1 {
                        count += 1;
                    }
                    else {
                        worker.push(c);
                    }
                    worker.push_str(p);

                }
                magazine = worker;
            }
            else {
                return false;
            }
        }

        return true;
    }

    assert_eq!(can_construct(String::from("a"), String::from("babbab")), true);
    assert_eq!(can_construct(String::from("a"), String::from("b")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);
}
