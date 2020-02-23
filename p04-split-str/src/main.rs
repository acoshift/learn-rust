fn main() {
    let s = split_str("hello world from hell");
    println!("{:?}", s);
}

fn split_str(s: &str) -> Vec<&str> {
    let mut r = vec![];
    let mut p = 0;
    for (i, &it) in s.as_bytes().iter().enumerate() {
        if it == b' ' {
            r.push(&s[p..i]);
            p = i + 1;
        }
    }
    r.push(&s[p..]);

    r
}
