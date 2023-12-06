use std::collections::HashSet;
fn main() {

    let mut num = HashSet::new();

    num.insert("apple");
    num.insert("mango");
    num.insert("banana")
    num.insert("apple");
    println!("Hash set data {:?}",num);
}
