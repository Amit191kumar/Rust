use std::io;

pub fn calculator_2() {
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
    fn exit(){
        println!("Please choose from 1 to 4")
    }
    print!("----------------------> Choose the operation you want to Perform <------------------------------------------------ \n 1--> Addation\n 2--> Subtraction\n 3--> Multiply \n 4 ---> Divide\n");

    let mut choose = String::new();
    println!("Please Enter the option between 1 to 4 :- ");
    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read from stdin");
    let choose: i32 = choose.trim().parse().expect("Please type a number!");
    
    
        
    

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

           

    if choose == 1{
        println!("The sum of two numbers is :- {}",add(n1,n2));
    }
    else if choose == 2{
        println!("The Substraction of two numbers is :- {}",sub(n1,n2));
    }
    else if choose == 3{
        println!("The Multiplication of two numbers is :- {}",mul(n1,n2));
    }else if choose == 4{
        println!("The division of two numbers is : - {}",div(n1, n2));
    }else{
        println!("Choose Between 1 to 4");
    }
}
