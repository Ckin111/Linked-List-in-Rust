// Loops are used to iterate until a conditon is met


pub fn run(){
    let mut count = 0;

    // infinite loop
    loop{
        count +=1;
        println!("Number: {}", count);

        if count == 20{
            break;
        }
    }

    // While loop fizzbuzz (loop through numbers 0 - 100 is its divisible by 3 print fizz 
    // if its divisible by 5 print buzz, both print fizzbuzz)
    let mut x = 0;
    loop{
        
        if x % 3 == 0 && x % 5 ==0 {
            println!("{} Fizzbuzz",x);
        } else if x % 3 == 0{
            println!("{} Fizz",x);
        } else if x % 5 == 0{
            println!("{} buzz",x);
        } else {
            println!("{}",x);
        }


        x+=1; //increments x
        if x ==101{
            break
        }
    }

    x = 0;

    while x<=100{
        if x % 3 == 0 && x % 5 ==0 {
            println!("{} Fizzbuzz",x);
        } else if x % 3 == 0{
            println!("{} Fizz",x);
        } else if x % 5 == 0{
            println!("{} buzz",x);
        } else {
            println!("{}",x);
        }

        x+=1; //increments x
    }

    // for range 
    
    for i in 0..100{
        if x % 3 == 0 && x % 5 ==0 {
            println!("{} Fizzbuzz",x);
        } else if x % 3 == 0{
            println!("{} Fizz",x);
        } else if x % 5 == 0{
            println!("{} buzz",x);
        } else {
            println!("{}",x);
        }
    }
}