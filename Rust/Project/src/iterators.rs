pub fn iterators_tut(){
    // let s = "abcde";
    // for i in 0..s.len(){
    //     println!("{}:{}",i,s.as_bytes()[i]);
    // }

    let v1 = vec![1, 2, 3, 6,];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}