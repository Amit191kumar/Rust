// Functions --> Used to store block of code for re-use

pub fn fun_ction(){
    greeting("Hello", "Rust");

// Bind functions values to variables 
let get_sum = add(5, 5);
println!("Sum of Two Numbers is :- {}",get_sum);

// Closures 
let n3:i32 = 2;
let add_nums =|n1:i32,n2:i32|n1  + n2 + n3;
println!("C sum: {}",add_nums(2,2));


}

fn greeting(greet:&str,name:&str){
    println!("{} {} , Welcome  to world of Programming $$$!!!",greet,name);

}

fn add(n1:i32,n2:i32)->i32{
    n1 + n2
}