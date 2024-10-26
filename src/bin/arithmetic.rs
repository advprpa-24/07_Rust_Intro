// Arithmetic Overflow
//
// 1. Run the program in DEBUG Mode. What happens?
// 2. Run the program in RELEASE Mode. What happens?
// 3. [optional] Try the following:
//    - checked_add() 
//    - wrapping_add()
//    - saturating_add()
//    - overflowing_add()

pub fn main() {
    add(1);
 }
 
 fn add(term: u8) {
     let b: u8 = 255; 
     let r: u8 = b + term;
     println!("Value: {}", r);
 }
 
// 4. Uncomment and compile the following function. What happens?
// cargo build --bin arithmetic.rs
// fn a() {
//     let b: u8 = 255; 
//     let r: u8 = b + 1;
//     println!("Value: {}", r);
// }
  