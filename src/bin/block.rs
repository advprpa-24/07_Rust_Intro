fn main() {
    let x = {
        let a = 5;
        let b = 10;
        a + b // The last expression of the block is returned
    };

    println!("The value of x is: {}", x);
}
