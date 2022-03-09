pub fn closur_tut(){
   let mut arr = [1,8,0,10,-78,45,2,99,56,100,10];
   println!("Arr is :- {:?}",arr);
    arr.sort();
    println!("Sorted arr is :- {:?}",arr);


let mut arr1 = [1,60,4,89,0,-10,-6,90,00];
use std::cmp::Ordering;
fn desc(a:&i32,b: &i32) -> Ordering{
    if a <b{Ordering::Greater}
    else if a > b { Ordering::Less }
else { Ordering::Equal }
}
arr.sort_by(desc);
print!(" New arr is :- {:?}", arr);
}
