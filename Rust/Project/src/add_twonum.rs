use std::io;


pub fn add(){

    let mut Num_1 = String::new();
    println!("Please Enter Value of Num_1 :- ");
    io::stdin()
        .read_line(&mut Num_1)
        .expect("Failed to read from stdin");
        let Num_1: i32 = Num_1.trim().parse().expect("Please type a number!");
    let mut Num_2 = String::new();
    println!("Enter The Value of Num_2:- ");
    io::stdin()
        .read_line(&mut Num_2)
        .expect("Failed to read from stdin");
        let Num_2: i32 = Num_2.trim().parse().expect("Please type a number!");

    println!(" Sum of {} and {} is :-  {}",Num_1,Num_2,Num_1 + Num_2);
    


}