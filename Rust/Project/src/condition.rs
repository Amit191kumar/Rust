pub fn conditionals_in_rust() {
    let age = 18;
    let check_id: bool = true;
    let cacha_vidhyak:bool = true;
    //if else
    if age > 18 && check_id || cacha_vidhyak {
        print!("You can Vote!!!");
    } else if age == 18 && check_id {
        println!("show me your Id")
    } else {
        print!("You cannot vote!!!!")
    } 
}
