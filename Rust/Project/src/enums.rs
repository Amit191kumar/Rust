// Enums are types which have a few definite values
enum Movement{
    // variable
    Up,
    Down,
    Left,
    Right
     
}

fn move_avatar(m:Movement){
    // Perform Action depending upon info
    match m {
        Movement::Up => println!("Avator is Moving Up"),
        Movement::Down => println!("Avator is Moving Down"),
        Movement::Left => println!("Avator is Moving Left"),
        Movement::Right => println!("Avator is Moving Right"),
    }
}

pub fn enums_tut(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}




