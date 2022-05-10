pub fn run(){
    // print to console
    println!("Hello from the print.rs file");

    // integers
    println!("Printing an integer {}",1);

    // basic formatting
    println!("{} is from {} and is {} years old!","Asher","Brooklyn",5);

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Asher", "Brooklyn", "code");

    // with parameters
    println!("{name} like to {activity}", name = "Asher", activity = "code");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // placeholder for debug trait
    println!("Printing a tuple: {:?}", (12,true,"hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}