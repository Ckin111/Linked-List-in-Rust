/* 
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory) (u means unsigned, not positive or negative)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is statically typed language, which means hta it must know the types of all the variables at compile time, 
// however, the compiler can usually infer what type we want to use based on the value and how we use it

pub fn run(){
    // default is i32
    let x= 1;

    // default is f64
    let y = 2.5;

    // add excplicit type
    let z: i64 = 34534534534534543;

    // find maxsize
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);

    // Boolean
    let is_active = true;
    
    // this is tup[le btw
    println!("{:?}", (x,y,z, is_active) );

    // Get boolean from expression
    let is_greater = 10>5;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x,y,z,is_active,is_greater,a1,face));

    

}