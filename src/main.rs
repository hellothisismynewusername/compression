use std::io::{Read, Write};
use std::process::exit;
use std::fs::File;

mod the_thing;
use the_thing::Bad;


fn main() {
    let mut file_names : Vec<String> = Vec::new();
    let mut compression : bool = true;
    let mut print : bool = false;
    for (i, arg) in std::env::args().enumerate() {
        if i >= 1 {
            match &*arg {
                "-d" => compression = false,
                "-v" => print = true,
                _ => file_names.push(arg.clone())
            }
        }
    }
    
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
            exit(1);
        }
        tmp.ok().unwrap()
    }).collect();

    println!("file_vec len {}. file name 0 {}.   waoidjdawioj {}", file_vec.len(), file_names[0], (&file_vec[0]).bytes().fold(0, |accum, _| accum + 1));

    if compression {

        let mut bytes : Vec<u8> = match the_thing::ball(file_vec) {
            Ok(x) => x,
            Err(Bad::Nothing) => {
                println!("unreachable");
                exit(1);
            },
            Err(Bad::TooLarge) => {
                println!("Files provided are too great in size");
                exit(1);
            },
            Err(Bad::IOError(e)) => {
                println!("{}", e);
                exit(1);
            },
            Err(Bad::Error(e)) => {
                println!("{}", e);
                exit(1);
            },

            _ => {
                println!("Something went wrong");
                exit(1);
            }
        };

        let mut write_tmp = File::create("tmp").unwrap();
        write_tmp.write_all(&mut bytes).unwrap();

        //the_thing::compress_and_write(&mut bytes, "out.crispied", true).unwrap();
    } else {

        let bytes_vec : Vec<u8> = match the_thing::decompress(&file_names[0]) {
            Ok(x) => x,
            Err(Bad::Nothing) => {
                println!("Empty file");
                exit(1);
            },
            Err(Bad::TooLarge) => {
                println!("File too large");
                exit(1);
            },
            Err(Bad::IOError(e)) => {
                println!("IO error {}", e);
                exit(1);
            },
            Err(Bad::Error(e)) => {
                println!("Error {}", e);
                exit(1);
            }
        };

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
