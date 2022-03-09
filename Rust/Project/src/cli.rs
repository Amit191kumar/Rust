use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Rust";
    let Status = "100%";
    //println!("Args : {:?}",args);
    //println!("Command : {}",command);
    if command == "hello"{
        println!("Hi {},Your are welcome",name);
         
    }else if command == "hey"{
        println!("Status is {}",Status);
    }else{
        println!("That is not a valid command");
    }
} 