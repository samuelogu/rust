pub fn run() {
    let x = 45;
    let y = 3.4;

    //find max size
    println!("Max i32: {}", std::i32::MAX);

    let is_active = true;

    let emoji = '\u{1F605}';

    println!("{:?}", (x, y, is_active, emoji));
}
