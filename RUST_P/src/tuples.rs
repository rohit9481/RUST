pub fn run(){
    let person: (&str,&str,i8) = ("Rohan","India",21);
    println!("{:?}",person);
    println!("{} is from {} and age is {}",person.0,person.1,person.2);
}