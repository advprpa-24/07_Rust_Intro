// Arrays
// 1. What do you expect for the second output? Run the program.
// 2. Try to change the first value in `a` to 10 after creation. Does it work?
pub fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    
    let b = ["RUST"; 10];
    println!("{:?}", b);
}

// 2. What's the problem here? Uncomment and compile.
// fn assignment() {
//     let mut a = [1, 2, 3, 4, 5];
//     let b = [1, 2, 3, 4];
//     a = b;
//     println!("{:?}", a);
// }



// 3. What's the problem here? Uncomment and compile.
// fn static_index() {
//     let a = [1, 2, 3, 4, 5];
//     let index = 10;
//     let element = a[index];
// }


// 4. What can happen, if the index is not statically known?
// You need to comment out the main function from the top of this file.

// use std::io;
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     // read index from user
//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//     let index: usize = index.trim().parse().expect("Invalid input");
//
//     // print the value at that index
//     println!("Value at index {}: {}", index, a[index]);
// }

