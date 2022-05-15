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
    println!("Containes 'World' {} ", hello_mut.contains("World") ); // this returns a boolean true or false
    
    // Replace some sub string
    println!("Replace: {}", hello_mut.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_mut.split_whitespace() {
        println!("{}",word);
    }
    
    println!("{}",hello_mut);
    // Create a string with a certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());
    println!("{}",s);
}