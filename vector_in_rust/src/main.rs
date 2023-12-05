fn main() {

    let mut my_vec: Vec<i32> = Vec::new();
    my_vec.push(10);
    my_vec.push(500);
    my_vec.push(100);
    my_vec.push(90);

 if let Some(ele)=my_vec.get(4)
 {
    println!("Vector Number {}",my_vec[4]);
 }
 else{
    println!("Element not found");
 }
  
    for  i in &my_vec{
        println!("{}" ,i);
    }
}
