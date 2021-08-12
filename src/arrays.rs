use std::mem;

pub fn run() {

    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    numbers[2] = 34;

    println!("{:?}", numbers[2]);

    //get stacked memory
    println!("Array has {:?} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

}
