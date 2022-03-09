// Primitive Strings ==> immutable fixed-length string somewhere in memory.
// String ==> Growable,heap allocated,data structure -use when you need to modify or own string data.

pub fn strings_rust(){
    let  name = "Rust "; // Primitive String
   
    println!("{}",name);
    // string
    let mut name1 = String::from("Hello!! ");
     // Push a character
     name1.push('R');
     //Push a String
     name1.push_str("ust");
    println!("{}",name1);

// Get Length
println!("Length of Name :- {}",name.len());
println!("Length of Name1 :- {}",name1.len()); 

// Capacity in bytes
println!("Capacity : - {}",name1.capacity());
println!("Is Empety :- {}",name.is_empty());

// Check for substrings using Contain
println!("Contains Rust {}",name.contains("hello"));

// Replace 
println!("Replace :- {}",name.replace("Rust", "Hello!!! Rust"));

// Loop through the strings by whitespaces
for Hello in name1.split_whitespace(){
    println!(" White Space :- {}",Hello);

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('R');
    s.push('U');
    s.push('S');
    s.push('T');
    println!(" String  After Pushing is ==> {}",s);

// Assertion Testing
assert_eq!(4,s.len());
assert_eq!(10,s.capacity());
println!("Assertion is :==> {}",s);
println!("Capacity  is :==> {}",s);

}



}