//Arrays:- fixed list where elements are of same data types.
/*  An array is a collection of elements of the same type allocated in a contiguous memory(or  Homogenous) block.
array are fixed in size  so we cannot perform push operation*/
use std::mem;
pub fn arrays_tut(){
    let mut number: [i32;10] = [1,2,3,4,5,6,7,8,9,10];
    // To assign a new value
    number[0] = 10;
    number[1] = 20;
    number[2] = 30;
    number[3] = 40;
    number[4] = 50;
    println!(" Array :- {:?}",number);
    // to Get single value
    println!("{}",number[0]);
    println!("{}",number[1]);
    println!("{}",number[2]);
    println!("{}",number[3]);
    println!("{}",number[4]);
    // to get array length
    println!("Length of array is:-  {}",number.len());

    // Array are stack allocated
    println!("The Array occupied {} bytes",mem::size_of_val(&number));

    // Get slice
    let slice: &[i32] = &number[0..4];
    println!("slice:- {:?}",slice);
// Cannot perform push operation on Array.
    number.push(70);
    println!("{:?}",number);
// Cannot perform pop operation on Array.
    number.pop();
    println!("Values of Vectors after pop :- {:?}",number);
}