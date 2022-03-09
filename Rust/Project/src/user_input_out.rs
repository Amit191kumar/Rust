use std::io;

pub fn i_o(){
    let mut input = String::new();
    println!("Please Enter a String:- ");

    match io :: stdin().read_line(&mut input){
        Ok(_) => {
            println!("Success! You Entered: {}",input.to_uppercase());
        },
        Err(e) => println!("Oops!! Something went Wroung {}",e)
    }

}