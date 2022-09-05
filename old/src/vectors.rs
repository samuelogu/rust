use std::mem;

pub fn run() {

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    numbers[2] = 34;

    numbers.push(29);

    println!("{:?}", numbers[2]);

    //get stacked memory
    println!("Vector has {:?} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    for i in &numbers {
        println!("{}", i);
    }

    for x in numbers.iter_mut() {
        *x += 2;
    }

    println!("{:?}", numbers)

}
