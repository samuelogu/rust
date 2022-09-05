pub fn run() {
    let name = "Samuel Ogu";
    let mut age = 29;
    println!("My name is {} and i'm {} years old", name, age);
    age = 30;
    println!("This year i'll be {} years old", age);

    //define const
    const ID: i32 = 002;
    println!("ID: {}", ID);

    //assigning multiple variables
    let (my_name, my_age) = ("Samuel", 30);
    println!("My name is {} and i'm {} years old", my_name, my_age);

}
