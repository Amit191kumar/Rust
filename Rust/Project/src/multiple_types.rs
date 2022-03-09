// Using an Enum to Store Multiple  data Types
/* Fortunately, the variants of an enum are defined under the same enum type, 
so when we need to store elements of a different type in a vector, we can define and use an enum! */
use std::fmt::Debug;
use std::fmt::Display;

pub fn multiple_data(){
    #[derive(Debug )]
    enum data{
        Int(i32),
        Float(f64),
        Text(String),}
    let mut show = vec![
        data::Int(20),
        data::Float(10.33),
        data::Text(String::from("Red")),

    ];
    println!("{:?}",show[1]);
}