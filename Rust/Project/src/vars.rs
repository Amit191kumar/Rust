/*1.Varibles holds Primitive data or Refrences to Data.
==> Primitive Data Types: A primitive data type is pre-defined by the programming language.
 The size and type of variable values are specified,
  and it has no additional methods.(eg - integer,boolean,bye short,int, long,float)
 ==> Non-Primitive Data Types: These data types are not actually defined by the programming language
 but are created by the programmer.(eg--> String,array etc)
2.variables are immutiable by default.
3.Rust is a block scoped Language.*/


pub fn variables_(){
    
    let name = "Amit";
    let mut age = 22;
    age = 23;
    println!("My name Is  {} and i am {} Years old",name,age);

// Defining a constant 
const ID:i32 = 001;
println!("ID:{}",ID);

// Multiple Variables
let (my_name,my_age) = ("Amit",23);
print!("{} is of {} Years",my_name,my_age);

}