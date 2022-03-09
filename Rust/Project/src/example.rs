use std::io;
pub fn run() {
    // Scanning the option from the user
    println!("**********************    Welcome To RUST Calculator  ******************\n");

    print!("----------------------> Choose the operation you want to Perform <------------------------------------------------ \n 1--> Addition\n 2--> Subtraction\n 3--> Multiplication \n 4 ---> Divide\n 5---> Square\n");
    while true{
    let mut choice = String::new();
    println!("Please Enter the option between 1 to 5 :- ");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    if choice > 6 || choice <=0 {
        println!("Choose Between 1 to 5 Only!!!!")
    } else {
        // First Number Input
        let mut n1 = String::new();
        println!("Please Enter Value of Number_1 :- ");
        io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read from stdin");
        let n1: i32 = n1.trim().parse().expect("Please type a number!");
        // Second Number Input
        let mut n2 = String::new();
        println!("Please Enter Value of Number_2 :- ");
        io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read from stdin");
        let n2: i32 = n2.trim().parse().expect("Please type a number!");
        match choice {
            1 => println!("sum of {} and {} numbers is:-  {}",n1,n2 , n1 + n2),
            2 => println!("Sub of {} and {} is :- {}",n1,n2, n1 - n2),
            3 => println!("Division of {} and {} numbers is :- {}",n1,n2, n1 * n2),
            4 => println!("Division of {} and {} numbers is :- {}",n1,n2, n1 / n2),
            5 => println!("square  of  {} is :- {}",n1, n1 * n1),
            6 =>{break;},
            _ => println!("Not valid Choose between 1 to 5 only"),
             
        }
        
        
    }
}
}
