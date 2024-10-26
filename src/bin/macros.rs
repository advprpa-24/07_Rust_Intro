// Macros!
fn main() {
    let i = 5;
    // prints a line to standard output, applying formatting described in std::fmt
    // inline and positional parameters
    println!("{i} {} {}", 4, true);

    // named parameter
    println!("Value: {value}", value = 4);

    // https://doc.rust-lang.org/std/fmt/
    println!("Hello {:-<5}!", "🦀");

    // works just like println! but returns the result as a string
    let s = format!("{i} {}", 4);
    println!("{s}");

    let t = (1, true);
    // logs the value of the expression and returns it
    let tt = dbg!(t);

    // Tuples do not implement the Display trait, use {:?} to print the Debug string
    println!("{:?}", tt);
}
