// 1. Run this program
// 2. Remove the type annotation. Does it still compile?
// 3. Try to modify the value of variable number to 6 after initialization. Does it work?
// 4. Read the compiler error and modify the declaration such that the variable can be reassigned.
// 5. Try to print the variable number without initializing it. Does it work?
// 6. Try to declare another variable with the same name. Does it work? Can the type be changed?
fn main() {
    // The following line declares and initializes a variable:
    let number: i32 = 5;
    // number = 6;
    println!("Value: {}", number)
}
