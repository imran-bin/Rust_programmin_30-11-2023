struct Point{
    x:i8,
    y:i8
}

impl Point{
    fn display(a:i8,b:i8)->Point{
        Point{x:a,y:b}
    }
    fn show(&self){
        println!("X is {} and y is {} ",self.x,self.y);
    }

}

fn main() {
    let p1 = Point::display(10,20);
    
    p1.show();
}
