//Structs --> used to create custom data types
// Traditional Struct

// struct Color{
//     red: u8,
//     green : u8,
//     blue : u8,

// }

// Tuple Struct
// struct Color(u8,u8,u8);

struct Person{
    first_name: String,
    middle_name:String,
    last_name : String,


}

impl Person{
    // Construct person
    fn new(first:&str,middle:&str,last:&str) -> Person{
        Person{
            first_name: first.to_string(),
            middle_name: middle.to_string(),
            last_name: last.to_string()
        }
    }

// Get Full name

fn full_name(&self) -> String{
    format!("{} {} {} ",self.first_name,self.middle_name,self.last_name)
}
     

// set last name 
fn set_first_name(&mut self, first: &str){
    self.first_name = first.to_string();
}

// set Name to Tuple

fn to_tuple(self) ->(String,String,String){
    (self.first_name,self.middle_name,self.last_name)
}
}

pub fn struct_tut(){
//     let mut c = Color{
//         red:255,
//         green:0,
//         blue:0,
//     };
//     c.red = 100;
//     println!("Color stored are : {} {} {}",c.red,c.green,c.green);
    

// let mut c = Color(255,0,0);
// c.1 = 200;
// println!("color in Tuples are : {} {} {}",c.0,c.1,c.2);

let mut p = Person::new("Captain","Jack","Sparrow");
println!("Person name is :-  {} {} {}",p.first_name,p.middle_name,p.last_name);

println!("Person full Name is :- {}",p.full_name());
p.set_first_name("Lord");
println!("Person After Changing First Name :-  {}",p.full_name());
println!("Person Name in Tuple {:?}",p.to_tuple());
 } 