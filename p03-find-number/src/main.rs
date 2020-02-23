fn main() {
    let arr = [1, 2, 5, 3, 2, 7, 8, 9, 2];
    let p = find_number(&arr[..], &7);
    println!("found index {} from {:?}", p, arr);
}

fn find_number(s: &[i32], p: &i32) -> i32 {
    for (i, it) in s.iter().enumerate() {
        if it == p {
            return i as i32;
        }
    }

    -1
}
