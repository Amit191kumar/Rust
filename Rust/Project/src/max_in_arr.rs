// find the Largest Number in a Array

pub fn run(){
    print!("<-------------Program to Find Largest element in an Array----------->\n");
    let a = [5,7,9,2,0,10];
    print!("Length of Given Array is :- {} \n ",a.len());
    let mut max:i32 = 0;
    let mut i:usize = 0;
    max = a[i];
    while i < a.len(){
        if max < a[i]{
            max = a[i];
        }
        i += 1;

    }
    println!("Largest Element in the Array is :- {}\n ",max);
}

// Swap The two Number
pub fn swap_no(){
    print!(" <----------------Program of  Swapping Two Numbers-------------->\n ");
    let mut a = 10;
    let mut b = 20;
    a = a+b;
    b = a-b;
    a = a-b;
    print!("Before swapping  {0} and {1} is  {2} and {3} \n ","a","b","10","20" );
    println!("After Swapping The Values of {0} and {1} is :- {2} and {3}","a","b",a,b);
}

// Place Holders

pub fn Place_Holder(){
    print!("**************************Program To Print Binary Hex and Oct*******************\n\n\n");

println!("binary: {:b} hex: {:x} Oct: {:o}\n\n",10,10,10);

print!("------------------------To Put Multiple Values Or To create a Tuple----------------------------\n\n\n ");
println!("{:?}",(10,"Hello!!","Rust",true,false,"\n\n\n"));

print!("***************************To Perform Basic Maths Operations********************\n\n");
println!("Addation of 10 + 10 = {}",10+10);
}