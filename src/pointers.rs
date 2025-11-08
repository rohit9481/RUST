pub fn run(){
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("arr: {:?}",(arr1,arr2)); 

    println!("---------------------");

    let vec1 = vec![1,2,3];
    let vec2= &vec1;
    //println!("vec: {:?}",(vec1,vec2)); // this will throw an error because vec1 is moved to vec2
    println!("vec: {:?}",(&vec1,vec2)); // we can use reference to avoid 
}