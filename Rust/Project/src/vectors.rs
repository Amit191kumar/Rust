// Vectors are Resizable Arrays
use std::mem;
pub fn vectots_tut(){
    let mut vector:Vec<i32> = vec![1,2,3,4,5];

    // to assign new values to vectors
    vector[0] = 10;
    vector[1] = 20;
    vector[2] = 30;
    vector[3] = 40;
    vector[4] = 50;
    println!(" Vector :- {:?}",vector);
    // to Get single value of Vectors
    println!("{}",vector[0]);
    println!("{}",vector[1]);
    println!("{}",vector[2]);
    println!("{}",vector[3]);
    println!("{}",vector[4]);
    // to get vector length
    println!("Length of vector is:-  {}",vector.len());

    // vector are stack allocated
    println!("The vector occupied {} bytes",mem::size_of_val(&vector));

    // Get slice
    let slice: &[i32] = &vector[0..4];
    println!("slice:- {:?}",slice);

    // to push a value in a variable
    vector.push(60);
    vector.push(70);
    vector.push(80);
    println!("Value of Vector after pushing an elements:- {:?}",vector);

    // Pop method in vectors
    vector.pop();
    println!("Values of Vectors after pop :- {:?}",vector);

    // Loop Through Vectors value
    for x in vector.iter(){
        println!("Values are : {}",x);
    }

    // Loop and mutate the values in the vectors
    for x in vector.iter_mut(){
        *x *= 2;
        println!("Vectors after multiplying by 2 :- {}",x);
        
    }
}
