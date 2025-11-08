pub fn new(){
    println!("Hello, world!");

    let is_great: bool = 10 >5;
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}",(is_great,a1,face));
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("{},{},{}",is_great,a1,face);
}