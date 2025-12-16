use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("1: {}, 2: {}", args[1], args[2]);

    // let mut items = [args[1].clone(), args[2].clone()];
    // items.sort();
    // println!("{}", items[0] == items[1]);
    // println!("{}", is_permutation(&args[1], &args[2]));
    println!("{}", inc_dec(&args[1], &args[2]));
}

// calc O(2n)
// mem  O(2n)
fn is_permutation(str1: &str, str2: &str) -> bool {
    let char_set1: [usize; 128] = inc(&str1);
    let char_set2: [usize; 128] = inc(&str2);

    char_set1 == char_set2
}

fn inc(str: &str) -> [usize; 128] {
    let mut char_set: [usize; 128] = [0; 128];
    for byte in str.bytes() {
        let index = byte as usize;
        char_set[index] += 1;
    }
    return char_set;
}

// answer
fn inc_dec(str1: &str, str2: &str) -> bool {
    let mut char_set: [isize; 128] = [0; 128];
    for byte in str1.bytes() {
        let index = byte as usize;
        char_set[index] += 1;
    }

    for byte in str2.bytes() {
        let index = byte as usize;
        // if index >= 128 {
        //     return false;
        // }
        char_set[index] -= 1;
        if char_set[index] < 0 {
            return false;
        }
    }

    return true;
}
