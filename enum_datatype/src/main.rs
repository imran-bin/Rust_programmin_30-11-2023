enum Deriction{
    Up,
    Down,
    Left,
    Right,

}
fn main() {
    let deriction:Deriction=Deriction::Up;

    match deriction{
        Deriction::Up => println!("Moving up"),
        Deriction::Down => println!("Moving down"),
        Deriction::Left => println!("Moving left"),
        Deriction::Right => println!("Moving right"),
    }

     
}
