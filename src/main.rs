use std::io::{Read, Write};
use std::process::exit;
use std::fs::File;

mod the_thing;

fn main() {
    let mut file_names : Vec<String> = Vec::new();
    let mut compression : bool = true;
    let mut print : bool = false;
    for (i, arg) in std::env::args().iter().enumerate() {
        if i >= 1 {
            match &arg {
                "-d" => compression = false,
                "-v" => print = true,
                _ => file_names.push(arg.clone())
            }
        }
    }
    let mut readfiles = Vec::new();
    if file_names.is_empty() {
        println!("No input file(s) provided");
        exit(1);
    }
    if !compression && file_names.len() != 1 {
        println!("Provide only 1 file for decompression");
        exit(1);
    }


    let mut file_vec : Vec<File> = file_names.iter().map(|name| {
        let tmp = File::open(name);
        if tmp.is_err() {
            println!("Error opening '{}'", name);
        }

        Ok(tmp)
    }).collect();

    if compression {

        let bytes : Vec<u8> = match the_thing::ball(file_vec) {
            Ok(x) => x,
            Err(Bad::Nothing) => println!("unreachable"); vec![0],
            Err(Bad::TooLarge) => println!("Files provided are too great in size"); vec![0],
            Err(e) => println!("{}", e); exit(1); vec![0]
        }
    } else {

    }

}

/*
fn print_u8_vec(inp : &Vec<u8>) {
    println!();
    print!("{{");
    for i in inp {
        print!("{}, ", i);
    }
    print!("}}");
}
*/
