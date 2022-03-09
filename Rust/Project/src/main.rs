// fn main(){
//     let mut i = 1;
//     while i<=4
//     {

//         //println!("{ }",i*i);
//         print!("{} ",i  *  i);
//         i = i + 1;
//     }
// }
// fn main(){
//     let mut i = 0;
//     while i <= 50{
//         print!("{} ", i);
//         i += 1;
//         if i%3 == 0 { continue; }
//         if i * i > 400 { break; }
//         print!("{} ", i * i);

//     }
// }
// fn main(){
//     print!("This is for Loop :-   ");
//     for i in 1..11{
//         print!("{  } ",i);
//     }
// }
// Largest Element in an Array

// fn main(){
//     print!("From 1 to 10 using for loop  ");
//     let x = 10;
//     for x in 0..5 {print!("{ }  ",x);}
//     print!(":{}",x);}
        
    
// fn main(){
//     let a = [1,5,0,-5,90,-78,89];
//     print!("Length of the given array is :-  {} \n ",a.len());
//     let mut max :i32 = 0;
//     let mut i:usize = 0;
//     max = a[0];
//     while i < a.len(){
//         if max < a[i]{
//             max = a[i];
//         }
//         i += 1;
//     }
//     println!("Largest Element in the Array is :- {}\n ",max);
// }


// Functions is Rust.
// fn main(){
//     fn say_hi(){
//         print!("Hello Function !!!");
//     }
//     say_hi();
// }

// Add Two numbers using Functions
// fn main(){
//     fn Add(num1:i32,num2:i32)-> i32{
//         return (num1+num2);
//     }
// }
// fn main(){
//     let a = [5,7,9,2,0,10];
//     print!("Length of Given Array is :- {} \n",a.len());
//     let mut max:i32 = 0;
//     let mut i:usize = 0;
//     max = a[i];
//     while i < a.len(){
//         if max < a[i]{
//             max = a[i];
//         }
//         i += 1;

//     }
//     println!("Largest Element in the Array is :- {} ",max);
// }

// fn main(){
//     let mut a = 10;
//     let mut b = 20;
//     a = a+b;
//     b = a-b;
//     a = a-b;
//     print!("Before swapping  {} and {} is  {2} and {3} \n ","a","b","10","20" );
//     println!("After Swapping The Values of {} and {} is :- {} and {}","a","b",a,b);
// }





//mod max_in_arr;
//mod vars;
//mod d_types;
//mod strings;
//mod tuples;
//mod Arrays;
//mod vectors;
//mod condition;
//mod loops;
//mod functions;
//mod pointers_ref;
//mod struct_;
//mod enums;
//mod cli;
//mod user_input_out;
//mod add_twonum;
//mod calc;
//mod calc2;
//mod example;
//mod clousers;
//mod iterators;
//mod multiple_types;
//mod generics;
//mod traits;
mod traits_exmp;
fn main(){
    //max_in_arr::run();
    //max_in_arr::swap_no();
    //max_in_arr::Place_Holder();
    //vars::variables_();
    //d_types::data_types();
    //strings::strings_rust();
    //tuples::tuples_tut();
    //Arrays::arrays_tut();
    //vectors::vectots_tut();
    //condition::conditionals_in_rust();
    //loops::loop_tut()
    //loops::fiz_buzz();
    //loops::f_loop();
    //functions::fun_ction();
    //struct_::struct_tut();
    //enums::enums_tut();
    //cli::run();
    //user_input_out::i_o();
    //add_twonum::add();
    //calc::calculator();
    //calc2::calculator_2();
    //example::run();
    //clousers::closur_tut();
    //iterators::iterators_tut();
    //multiple_types::multiple_data();
    //generics::generics_tut();
    //traits::traits_tut();
    //generics::generics_tut_01();
    traits_exmp::traits_prg();
}
