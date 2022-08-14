/* Finding the nth Fibonacci Number using dynamic programming
 and space optimization */

use std::io;
fn main(){
    let mut x: u32 = 0;
    let mut y: u32 = 1;
    let mut ret: u32 =0;
    let mut term : String = String::new(); 
    println!("Enter term you want to find:");
    io::stdin().read_line( &mut term).unwrap();
    let term : i32 = term.trim().parse().unwrap();
    if term < 0{
        println!("Wrong input");
    }
    else if term == 0{
        println!("{x}");
    }
    else if term == 1{
        println!("{y}")
    }
    else{
        for _val in 2..term + 1{
            let z : u32 = x + y;
            x = y;
            y = z;
            ret = y
          
        }
        println!("{ret}") 
    }
}