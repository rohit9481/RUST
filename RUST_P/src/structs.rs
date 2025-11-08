struct Person{
    first_name: String,
    last_name: String,
}

impl Person{
    fn new(first_name: &str,last_name: &str)->Person{
        Person{
            first_name:first_name.to_string(),
            last_name:last_name.to_string(),
        }
    }

    //create a full_name
    fn full_name(&self)->String{
        format!("{} {}",self.first_name,self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str){
        self.last_name = last_name.to_string();
    }
}

pub fn run(){
    let mut p = Person::new("John", "Doe");
    println!("First Name: {}", p.first_name);
    println!("Last Name: {}", p.last_name);
    println!("Full Name: {}", p.full_name());
    p.set_last_name("smith");
    println!("updated last name:{}",p.full_name());
}
