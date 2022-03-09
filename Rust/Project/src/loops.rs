// pub fn loop_tut() {
//     let mut x = 0;
//     loop {
//         x = x + 1;
//         println!("value of x is :-  {}", x);
//         if x == 10000{
//         break;

//         }
//     }
// }

// While Loop (FizzBuzz Program)
// pub fn fiz_buzz(){
//     let mut count = 0;
// while count <= 100{
//     if count % 15 == 0{
//         println!("Fizz_Buzz");
//     }else if count % 5 == 0{
//         println!("Fizz");

//     }else if count % 3 == 0{
//         println!("Buzzzzz");
//     }else {
//         println!("{}",count); 
//     }
// // Increment
// count = count +1;    
// }
// }

// for range loop
pub fn f_loop(){
    for x in 0..100{
        if x % 15 == 0{
            println!("Fizz_Buzz");
        }else if x % 5 == 0{
            println!("Fizz");
    
        }else if x % 3 == 0{
            println!("Buzzzzz");
        }else {
            println!("{}",x); 
        }

    }
}



