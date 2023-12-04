fn main() {

    let nam : &str = "BN";

    let state = match nam{
        "BN"=> {println!("Match for MW");"Bangladesh"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("Sate name is {}",state);
}
