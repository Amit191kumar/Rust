use std::io;
pub fn calculator() {
    fn add(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }
    fn sub(n1: i32, n2: i32) -> i32 {
        n1 - n2
    }
    fn mul(n1: i32, n2: i32) -> i32 {
        n1 * n2
    }
    fn div(n1: i32, n2: i32) -> i32 {
        n1 / n2
    }

    print!("----------------------> Choose the operation you want to Perform <------------------------------------------------ \n 1--> Addation\n 2--> Subtraction\n 3--> Multiply \n 4 ---> Divide\n");
    // Scanning the option from the user
    while true{
    let mut choose = String::new();
    println!("Please Enter the option between 1 to 4 :- ");
    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read from stdin");
    let choose: i32 = choose.trim().parse().expect("Please type a number!");

    if choose > 4 || choose<=0{
        println!("Choose Between 1 to 4");
    } else{

    // Taking Input From User
    let mut n1 = String::new();
    println!("Please Enter Value of Number_1 :- ");
    io::stdin()
        .read_line(&mut n1)
        .expect("Failed to read from stdin");
    let n1: i32 = n1.trim().parse().expect("Please type a number!");

    let mut n2 = String::new();
    println!("Please Enter Value of Number_2 :- ");
    io::stdin()
        .read_line(&mut n2)
        .expect("Failed to read from stdin");
    let n2: i32 = n2.trim().parse().expect("Please type a number!");
  if choose == 1 {
            println!("The Sum  is :- {} ", add(n1, n2));
        } else if choose == 2 {
            println!("The sub is :- {}", sub(n1, n2));
        } else if choose == 3 {
            println!("The mul is :- {}", mul(n1, n2));
        } else if choose == 4 {
            println!("The div is :- {}", div(n1, n2));
        } else {
            println!("choose between 1 to 4 Thank You!!!!");
        }
    }
}
}
