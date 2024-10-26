// Functions
fn main() {
    let res = ex(true, 4);
    println!("Result {}", res);
}

fn ex(b: bool, i: i32) -> i32 {
    let default_val = 12;
    if b { i } else { default_val }
}