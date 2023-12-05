 use std::fs::File;
fn main() {

    let result = is_even(13).unwrap();
    // let f=File::open("a.text").expect("file not found");

    println!("Result is {}",result);
       let n =

}

fn is_even(no:i32)->Result<bool,String>{
    if no%2==0{
        return Ok(true);
    }
    else{
          return Err("NOT_AN_EVEN".to_string());
    }
}
