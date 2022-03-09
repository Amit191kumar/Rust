/* Generics are a facility to write code for multiple contexts with different data types. 
In Rust, generics refer to the parameterization of data types and traits. 
Generics allows to write more concise and clean code by reducing code duplication and providing type-safety.
The concept of Generics can be applied to methods, functions, structures, enumerations, collections and traits.*/

pub fn generics_tut(){

struct Data<T>{
    value:T,
}

    // Generic Type of i32
    let a:Data<i32> = Data{value :350};
    println!("Value  of integer is : {} ",a.value);

    //Generic type of String

    let b:Data<String> = Data{value:"Jerry".to_string()};
    println!("Value of String type is : {}",b.value);
    // Generic Type of Float
    
    let c:Data<f32> = Data{value:10.20};
    println!("Value of Float type is : {}",c.value);

}
use std::fmt::Display;

pub fn generics_tut_01(){
    values_print(10 as u8);
    values_print(20 as u16);
    values_print(20.333 as f32);
    values_print("Hello!!! I am Rust Generics");

}
fn values_print<T:Display>(t:T){
    println!("Inside values_print Generic function: ");
    println!("{}",t);
}