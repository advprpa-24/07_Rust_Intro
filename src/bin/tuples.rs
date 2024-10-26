// Tuples
// 1. Run this program
// 2. Try to change the second component to `false` after the first println!
//    Does it work?
pub fn main() {
    let a: (i32, bool) = (42, true);

    println!("{:?}", a);

    let (answer, is_correct) = a;

    if is_correct {
        println!("Answer: {answer}");
    }
    
    println!("{}", a.1);
}