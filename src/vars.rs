// Variables hold primitive data or references to data
// Variables are immutable (cant change them) by default
// Rust is block-scoepd language

pub fn run(){
    let name = "Asher";
    let mut age = 5;
    age = 6;

    println!("My name is {} and I am {}", name, age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}",ID);

    // assign multiple variables at once
    let (my_name, my_age) = ("Brad",37);
    println!("{} is {}", my_name, my_age);
}