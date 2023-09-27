fn main() {
    pub fn is_valid(s: String) -> bool {
        let mut worker = s.clone();
        let mut holder = String::new();
        let mut valid = true;

        let left = ['(','{','['];

        while !(worker.is_empty() && holder.is_empty()) {
            let c = worker.pop();


            match c {
                Some(l) => if left.contains(&l) {

                    let c2 = holder.pop();
                    match c2 {
                        Some(r) => if !((l == '[' && r == ']') || (l == '(' && r == ')') || (l == '{' && r == '}')) {
                            valid = false;
                            break;
                        },
                        None => {
                            valid = false; 
                            break;
                        },
                    }
                } else {
                    holder.push(l);
                },
                None => {
                    valid = false;
                    break;
                },
            }
        }

        return valid;
    }

    assert_eq!(is_valid(String::from("]")), true);

}
