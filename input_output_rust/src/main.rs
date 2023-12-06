
use std::io;
fn main() {
    println!("Enter your name: ");

    // let mut name=String::new();
    let mut name=String::new();
    io::stdin().read_line(&mut name);
    println!("Name is : {}",name);

    // convert another type

    let num:i32=match name.trim().parse(){
        Ok(num)=> num,
        Err(_)=>{
            println!("Invalid input plese inter a valid i32 number");
            return;
        }
    };
    println!("You entered: {}", num);

}
