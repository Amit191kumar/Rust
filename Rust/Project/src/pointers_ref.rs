// Refrenece Pointers --> Point to a resource in memory

pub fn poin_ters(){
    //Primitive Array
let arr1 = [1,2,3];
let arr2 = arr1;
println!("Values  in  Array is :-  {:?}",(arr1,arr2));


/* With non-primitives,if you assign another variable to a piece of data,the first
 variable will no longer  hold the value.you'll need to use a reference(&) to point the resource. */


// Vector
let vec1 = vec![1,2,3];
let vec2 = &vec1;
println!("Values in Vectors  :- {:?}",(&vec1,vec2));




}

