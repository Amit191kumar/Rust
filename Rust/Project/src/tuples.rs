/*   ## ---> A tuple in rust is a finite heterogeneous compound data type, meaning it can store more than one value at once. 
In tuples there is no inbuilt method to add elements into a tuple. We can use the index to get the value of a tuple,
and we also can not iterate over a tuple using for loop.
 
 ## ---> It is important to note that tuples are a sequence in Rust. 
 This means its elements can be accessed by the position which is also known as tuple indexing.
 
 ## we can use strings and integers both in tuples. */



pub fn tuples_tut(){
    let person:(&str,&str,i8) =("Amit","Kumar",001);
    println!("My name is {} {} and my id is {} ",person.0,person.1,person.2);
    for i in person 0..person.len(){
        println!("{}",i);
    }
}