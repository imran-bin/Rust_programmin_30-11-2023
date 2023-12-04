fn main() {

    // let num =is_even(56);
    match is_even(50){
        Some(data)=>{
            if data ==true{
                println!("Number is even");
            }
        },
        None=>{
            println!("Not even");
        }
        
    }
   
}

fn is_even(n:i8)->Option<bool>{
    if n%2==0{
        Some(true)
    }
    else{
        None
    }
}