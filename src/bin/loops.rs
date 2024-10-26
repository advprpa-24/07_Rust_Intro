// Loops
fn main() {
    println!("Looping over a range:");
    for i in 1..4 {
        // Range 1,2,3
        print!("{},", i);
    }


    println!("\nLooping over an array:");
    let numbers = [1, 2, 3];
    for &num in numbers.iter() {
        print!("{},", num);
    }


    println!("\nLooping as long a condition is true:");
    let mut count = 0;
    while count < 4 {
        print!("{}", count);
        count += 1;
    }


    println!("\nLooping until break:");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter >= 4 {
            break; // Exit the loop
        }
        print!("{}", counter);
    }


    println!("\nloop is an expression!");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // Break the loop and return the value
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
}
