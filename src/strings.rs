pub fn run() {

    let mut hello = String::from("Hello ");
    let emoji = '\u{1F605}';
    hello.push(emoji);
    hello.push_str(" from Nigeria");

    // println!("Length of string is {}", hello.len());

    // get capacity

    println!("{}", hello.capacity());
    println!("Replace: {}", hello.replace("Hello", "Hey"));
    println!("Contains the word 'Hello': {}", hello.contains("Hello"));
    println!("Is empty {}", hello.is_empty());

    for word in hello.split(" ") {
        println!("{}", word);
    }


    println!("{}", hello);
}
