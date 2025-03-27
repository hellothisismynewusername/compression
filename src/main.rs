use std::io::{Read, Seek, SeekFrom};
use std::process::exit;
use std::fs::File;

mod the_thing;
use the_thing::Bad;


fn main() {
    let mut file_names : Vec<String> = Vec::new();
    let mut compression : bool = true;
    let mut print : bool = false;
    let mut verbose : bool = false;
    for (i, arg) in std::env::args().enumerate() {
        if i >= 1 {
            match &*arg {
                "-d" => compression = false,
                "-v" => verbose = true,
                "-vv" => print = true,
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

    if print {
        println!("file_vec len {}. file name 0 {}.   waoidjdawioj {}", file_vec.len(), file_names[0], (&file_vec[0]).bytes().fold(0, |accum, _| accum + 1));
    }

    file_vec[0].seek(SeekFrom::Start(0)).expect("Seek error");

    if compression {

        if verbose || print {
            println!("Balling files");
        }

        let mut bytes : Vec<u8> = match the_thing::ball(file_vec, file_names, print) {
            Ok(x) => x,
            Err(Bad::Nothing) => {
                println!("Empty");
                exit(1);
            },
            Err(Bad::TooLarge) => {
                println!("Files provided are too great in size (limit is 4,294,967,295)");
                exit(1);
            },
            Err(Bad::IOError(e)) => {
                println!("{}", e);
                exit(1);
            },
            _ => {
                println!("Something went wrong");
                exit(1);
            }
        };

        // let mut write_tmp = File::create("tmp").unwrap();
        // write_tmp.write_all(&mut bytes).unwrap();

        if verbose || print {
            println!("Compressing");
        }

        the_thing::compress_and_write(&mut bytes, "out", print, verbose).unwrap();
    
    } else {

        if verbose || print {
            println!("Decompressing");
        }

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
            _ => {
                println!("Something went wrong");
                exit(1);
            }
        };

        if verbose || print {
            println!("Unballing");
        }

        match the_thing::unball_and_write(bytes_vec, print, verbose) {
            Ok(_) => {},
            Err(Bad::FromUtf8Error(e)) => {
                println!("Utf8Error {}", e);
            },
            Err(Bad::IOError(e)) => {
                println!("IOError {}", e);
            },
            Err(Bad::Nothing) => {
                println!("Empty file");
            },
            Err(Bad::TooLarge) => {
                println!("File too large");
            }
        }

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
