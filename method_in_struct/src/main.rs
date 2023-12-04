struct Rectangle{
    width:i8,
    height:i8,
}

impl Rectangle{
    fn area(&self)->i8{
        self.width * self.height

    }
}
fn main() {

    let rec1= Rectangle{
        width:2,
        height:4
    };
    

    println!("Result is {}", rec1.area());
     
}
