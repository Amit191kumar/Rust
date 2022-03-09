/* Primitive Data Types in Rust---
Integers (can have negative values) ----> i8,i16,i32,i64,i128  (Number is the number of bits they can take in the memory.) 
Unsigned (Means Cannot have Negative Values)--> u8,u16,u32,u64,u128
floats - f32,f64
Boolean (bool)
Characters (char)
Tuples
Arrays*/

/* Rust is a staticlly typed language,which means that it must know the types of all variables at compile time.
 However,the compiler can usually infer what type we want to use based on the value and how we use it*/


pub fn data_types()
{ // by Default it is "i32"
    let x = 10;
 // by default it is "f64" in float
    let y = 2.5;
 // add Explicit type
    let z:i64 = 458968973686897;
 // Find max size
 println!("Max for i32 {}",std::i32::MAX); 
 println!("Max for i64 {}",std::i64::MAX); 
 // Boolean
 let is_active :bool = true;
 let is_grater : bool = 2<5;
 // character
 let a1 = 'A';
 let emoji = '\u{1F600}';
 println!("{:?}",(x,y,z,is_active,is_grater,a1,emoji));
  }