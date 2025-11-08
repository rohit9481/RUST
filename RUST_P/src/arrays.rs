pub fn run(){
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("{:?}",numbers);

    println!("First number: {}",numbers[0]);

    let mut mutableArray: [i32;4] = [1,2,3,4];
    mutableArray[2] = 10;
    println!("mutableArray: {:?}",mutableArray);
}