#![allow(warnings)]
mod process;
use std::{env, fs::read};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        print!("Usage: hexview <file path> <output ascii file path>(opinion)");
        return;
    }
    let filename = &args[1];
    let data = process::read(filename.clone());
    let mut output_file = "No";
    if args.len() == 3{
        output_file = &args[2];
    }
    println!("Size: {}",data.len());
    process::print_hex(&data,output_file);
    print!("Args: {}",output_file);
}
