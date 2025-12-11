use std::collections::HashSet;

fn main() {
    let str: String = String::from("abcdefghix");
    // println!("is_duplicated: {}", is_duplicate_char_with_hash(&str));
    println!("is_duplicated: {}", is_duplicate_char_with_bool(&str));
}

// O(n)
fn is_duplicate_char_with_hash(str: &str) -> bool {
    let mut seen_values = HashSet::new();
    for item in str.chars() {
        if !seen_values.insert(item) {
            return true;
        }
    }
    return false;
}

// ascii前提
// O(n)
fn is_duplicate_char_with_bool(str: &str) -> bool {
    let mut char_set: [bool; 128] = [false; 128];
    for byte in str.bytes() {
        let i: usize = byte as usize;
        if char_set[i] {
            return true;
        }
        char_set[i] = true;
    }
    return false;
}
