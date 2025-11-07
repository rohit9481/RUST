mod print;
fn main() {
    print::run();
    println!("numbers {}", 1);
    println!("{} is from {}", "bread","Mass");
    println!("{0} is from {1} and {0} likes {2}", "bread","Mass","code");
    println!("{name} likes to play {game}", name="Alice", game="chess");
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("{:?}",(12,true,"Hello"));
    println!("10 + 10 = {}", 10+10);

}
