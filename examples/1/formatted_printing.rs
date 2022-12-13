fn main () {
    // {} will be automatically replaced with any argument 
    println!("{} days", 21);
    // using named arguments 
    println!("{subject} {verb} {object}", subject="The machine learning engineer", object="macbooks", verb="hated");
    // format specifying 
    let x = 69420;
    println!("Formatting value {} using different base representations", x);
    println!("Base 10 (decimal):                   {}", x);
    println!("Base 2  (binary):                    {:b}", x);
    println!("Base 8  (octal):                     {:o}", x);
    println!("Base 16 (hexadecimal lowercase):     {:x}", x);
    println!("Base 16 (hexadecimal uppercase):     {:X}", x);

    // right justify with specified width 
    println!("{number:>10}", number=1);
    // can pass width in as variable in formater by appending dollar sign $
    println!("{number:0>width$}", number=1, width=5);
    // rust checks for correct number of arguments 

    // wont work!
    // println!("My name is {0}. {1}, {0}", "Wilson")
    println!("My name is {0}. {1}, {0}", "Wilson", "Akin");
    
    /*
    only types implement fmt::Display can be formatted with {}. User-defined types do not implement 
    do not implement fmt::Display
    */ 

    #[allow(dead_code)]
    // integer 32 
    struct Structure(i32);

    /*
    wont compile since Structure does not implement fmt::Display 
    
    println!("This struct `{}` wont print...", Structure(3));
    */
    // primitive type float 64
    let number: f64=1.0;
    /*
    primitive type usize  - how many bytes it takes to reference any location in memory.
    e.g. 32 bit target, this is 4 bytes 
         64 bit target, this is 8 bytes
    */ 
    let width: usize=5;
    println!("{number:>width$}");

    // activities 
    let pi = 3.141592; 
    let dp = 3;
    println!("pi {val:.*} up to {0} decimal places", dp, val=pi);

}