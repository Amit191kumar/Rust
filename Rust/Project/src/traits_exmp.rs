
    pub trait Details{
    fn description(&self) -> String;
    fn launched_year(&self) -> i32;}

struct CAR{
    brand_name: String,
    color: String,
    year:i32
}

// Implementing an in-built trait details
// on the car struct

impl Details for CAR{
    // Method for returning overview of the car
    fn description(&self) -> String{
        return format!("I Have A {} Which is {} in color.",self.brand_name,self.color);
    }
    fn launched_year(&self) -> i32{
        return 2022 - self.year;}
    
} 


pub fn traits_prg (){
    let car = CAR{
        brand_name:"Swift".to_string(),
        color: "Black".to_string(),
        year:1992
    };
    let car2 = CAR{
        brand_name:"Wagnor".to_string(),
        color:"white".to_string(),
        year:1997
    };
    println!("{}",car.description());
    println!("The car was released {} years ago.\n",car.launched_year());

    println!("{}",car2.description());
    println!("The car was released {} years ago.",car2.launched_year());
}