// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data


pub fn run(){
    let hello = "hello"; // immutable
    let mut hello_mut = String::from("hello "); //mutable because of STRING:: and mut
    println!("{}", hello_mut);

    // get length
    println!("Length: {}",hello_mut.len());

    hello_mut.push('W');

    hello_mut.push_str("orld");

    println!("{}",hello_mut);

    // capacity in bytes
    println!("Capacity: {}",hello_mut.capacity());

    // check is empty
    println!("Is Empty: {}", hello_mut.is_empty());
 
    // Contains some sub string
        
    
    println!("{}",hello_mut);
}