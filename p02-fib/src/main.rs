fn main() {
    let x = fib(20);
    println!("fib = {}", x);
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut mem = (1, 1);
    for _ in 2..n {
        mem = (mem.1, mem.0 + mem.1);
    }
    mem.1
}
