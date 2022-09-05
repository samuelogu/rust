pub fn run() {
    println!("Hello from print.rs file");

    //basic formatting
    println!("{} is a good boy", "Samuel");

    //positional arguments
    println!(
        "{0} is from {1} and {0} loves to {2}",
        "Samuel", "Nigeria", "code"
    );

    //named arguments
    println!(
        "{name} likes to code {language}",
        name = "Samuel Ogu", language = "Rust"
    );
}
