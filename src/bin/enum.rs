// Enum and Pattern Matching
// 1. Add a value constructor named Delete which takes a bool `force`.
// 2. Run the compiler.
// 3. Extend the match expression.
// 4. Call process_message with a Delete argument.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m: Message = Message::ChangeColor(0, 10, 0);
    process_message(m);
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
