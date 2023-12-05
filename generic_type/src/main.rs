
struct Data<T>{
    info:T,
}
fn main() {
    let n:Data<i32>=Data{info:20};
    println!("Hello, world! {}",n.info);
    let a :Data<String>=Data{info:{String::from("mediusware ")}};
    println!("Data is {}",a.info);
}
