use std::{env, ops::Add, process::Command, io::Read};

fn main() {
    let args:Vec<String> = env::args().collect();
    let link: String = create_link(args[1].as_str());
    println!("{}",link);
}

fn create_link(cat: &str) -> String{
    let mut link:String = String::from("https://docs.rs/crate/");
    link = link.add(cat);
    link.add("/latest/download")
}
