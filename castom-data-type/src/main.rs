#[derive(Debug)]
struct Persion{
    name: String,
    age:i8,
    message:{&str,i8}
   }
fn main() {
   
   let name = String::from("imran khan");
   let age =25i8;

   let petter =Persion {name:name,age:age,message:{name,age}};

    println!("Hello, world! {}",name);
}
