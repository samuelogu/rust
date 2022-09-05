pub fn run() {

    let age = 34;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
        return;
    }

    if age < 21 {
        println!("Bartender: You can not drink in this bar");
    }

}
