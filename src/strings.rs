pub fn run(){
    //let _hello = "first string";
    //let mut newHello = String:: from("Hello new Word");//mutable and heap allocation string type
    // println!("{}",hello);
    
    // println!("{}",newHello);

    // newHello.push('!'); // adding a single char into the string
    // newHello.push_str(" How are you?"); // adding a string slice into the string
    // println!("{}",newHello);

    // println!("contains: {}", newHello.contains("new"));
    // println!("replace: {}:", newHello.replace("new","old"));

    // for word in newHello.split_whitespace(){
    //     println!("{}",word);
    // }

    let mut s = String::with_capacity(10);
    s.push_str("asdfghjk");
    println!("{} and {} and cap: {}",s,s.len(),s.capacity());

    assert_eq!(10,s.capacity());

} 