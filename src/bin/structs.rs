// Structs
// 1. Run this program.
// 2. Try to change the `width` to 50 after creation. Does it work?
// 3. Try to print the value `rect`. 
//    Can you make it work using the hints in the error msg?
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("width: {}, height: {}", rect.width, rect.height)
    // println!("{:?}", rect) // 3. print rect
}
