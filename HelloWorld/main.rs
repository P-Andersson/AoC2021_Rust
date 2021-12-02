fn get_str() -> String {
    return "AoC 2021".to_string();
}

fn main() {
    println!("{}",
            String::from("Ready for ")
             + &get_str() 
             + &String::from("!"));
}