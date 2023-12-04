fn display(emp:Employee){


    println!("my name is {} my age is {} and my salary {}" ,emp.name,emp.age,emp.salary);
}

struct Employee{

    name:String,
    age:i8,
    salary:i8,

}

fn main() {


    let mut emp1= Employee{
        name:String::from("imran khan"),
        age:25,
        salary:15
    };
    emp1.age=40;
    display(emp1);
    println!("Hello, world!");
}
