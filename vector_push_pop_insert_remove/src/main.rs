
fn main() {

    let mut fruits = vec!["apple","banana","orange","mango"];

    println!("Initial vector {:?}" , fruits);

    if let Some(last_fruit)=fruits.remove(3){
        println!("Fruits pop {}",last_fruit);
    }
    println!("After pop {:?}" , fruits);
//     fruits.insert(1,"grapes");
//     println!("after insert {:?}", fruits);

//    if let Some(ele) = fruits.remove(5){
//     println!("Remove fruits {:?}",ele);
//    }
//    else{
//     println!("Element not found");
//    }
}
