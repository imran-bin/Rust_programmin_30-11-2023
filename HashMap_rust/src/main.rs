use std::collections::HashMap;
fn main() {

    let mut my_book = HashMap::new();
    my_book.insert("Rust programming" ,40);
    my_book.insert("C++ programming",78);
    println!("HashyMap data {:?}",my_book);

    for (book,price) in &my_book{
        println!("Book Name: {} price: {}",book,price );
    }
    
    if let Some(price)=my_book.get("Rust programming"){
        println!("Price: {}",price);
    }
    else{
        println!("Data not found");
    }

    if !my_book.contains_key("C++ programming"){
        println!("C++ book is not found");
    }

    my_book.remove("Rust programming");

    println!("HashyMap data {:?}",my_book);

//   Multiple value setup on same key in hashmap

    let mut info= HashMap::new();

    info
     .entry("Imran")
     .or_insert(Vec::new())
     .push("satkhira");

    info 
     .entry("Imran")
     .or_insert(Vec::new())
     .push("587");

    println!("Information : {:?}",info);

    if let Some(key)=info.get("Imran"){
        println!("Values for imran {:?} ",info);
    }

    for (key ,value) in info{

        // println!("Key {:?} and value {:?}",key, value);
        for val in value{
            println!("key {:?} and values {:?}",key,val)
        }
    }

}
