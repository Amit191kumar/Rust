/* Traits can be used to implement a standard set of behaviors (methods) across multiple structures.
 #Traits are like interfaces in Object-oriented Programming.  
 # A trait tells the Rust compiler about functionality a particular type has and can share with other types. 
 Traits are an abstract definition of shared behavior amongst different types. So, we can say that traits 
 are to Rust what interfaces are to Java or abstract classes are to C++. 
 A trait method is able to access other methods within that trait.*/



pub fn traits_tut(){
    // Creating an instance of the structure
    
    let b1 = Book{
        id:101,
        name: "Programming With Rust"

    };
    b1.print();

}

// Declaring a Structure

struct Book{
    name: &'static str,
    id:u32
}
// Declareing a trait

trait Printable{
    fn print(&self);
}
//implementing the trait

impl Printable for Book{
    fn print(&self){

        println!(" Book  ID  is : {} \n and Name is : {}",self.id,self.name)
    }
}