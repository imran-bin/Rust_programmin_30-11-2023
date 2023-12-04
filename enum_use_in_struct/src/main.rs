#[derive(Debug)]
enum GenderType{
    Female,
    Male,
}
#[derive(Debug)]
struct Persion{
    name:String ,
    gender: GenderType,
}

impl Persion{
    fn display(&self){
        println!("Name: {:?} Gender {:?}",self.name,self.gender);

    }
}

fn main() {

    let p1 = Persion{
        name :String::from("mediusware"),
        gender:GenderType::Male,
    };
//   println!("{:?}",p1);
p1.display();
}
