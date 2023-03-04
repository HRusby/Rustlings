// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Mv {x:i32, y: i32}

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move {coords: Mv},
    Move2 {x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}


impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move2 { x: 20, y: 40},
        Message::Move { coords: Mv{x: 3, y: 4}},
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
