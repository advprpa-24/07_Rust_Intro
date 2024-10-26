const RESULT: u32 = fib(6);

const fn fib(x: u32) -> u32 {
    match x {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}

pub fn main() {
    println!("The value of RESULT is: {}", RESULT);
}