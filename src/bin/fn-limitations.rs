// No ad-hoc overloading!
fn main() {
    print(3);
    // print(true);
}

fn print(i: i32) {
    println!("{i}");
}

// Uncomment and compile.
// fn print(b: bool) {
//     println!("{b}");
// }
