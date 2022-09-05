use std::env;

pub fn run() {
    let name = "Samuel";
    let status = 100;
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    // println!("Arg: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you", name);
    }else if command == "status" {
        println!("Status is {}", status);
    }else {
        println!("That is not a valid command");
    }

}

