pub fn run(){

    let mut vectors: Vec<i32> = vec![1,2,3,4];
    vectors.push(5);
    vectors.push(6);
    println!("vectors: {:?}",vectors);

    vectors.pop();
    println!("after pop: {:?}",vectors);

    for x in vectors.iter(){
        println!("values:{}",x);
    }

    //loop and mutate
    for x in vectors.iter_mut(){
        *x *= 2;
    }
    println!("mutated vec: {:?}",vectors);
}