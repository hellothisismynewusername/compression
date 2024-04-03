use std::io::{Read, Write};
use std::process::exit;
use std::fs::File;

fn main() {
    let mut file_name : Option<String> = None;
    let mut compression : bool = true;
    let mut cntr = 0;
    for arg in std::env::args() {
        println!("{}", arg);
        if cntr == 1 {
            file_name = Some(arg.clone());
        }
        if cntr == 2 && arg == "-d" {
            compression = false;
        }
        cntr += 1;
    }
    if cntr > 3 || cntr < 2 {
        println!("Wrong number of arguments");
    }
    let mut readfile;
    if file_name.is_none() {
        println!("It didn't work");
        exit(1);
    } else {
        readfile = File::open(file_name.clone().unwrap()).expect("Couldn't open up '{file_name}'");
    }

    if compression {
        let mut writefile = File::create(file_name.unwrap() + ".crispyfries").expect("Making writefile failed");
        let mut file_buf : Vec<u8> = Vec::new();
        let file_length = readfile.read_to_end(&mut file_buf).expect("Reading into buffer in compression mode didn't work");
        print_u8_vec(&file_buf);
        let mut cntr : u64 = 0;
        let mut new_buf : Vec<u8> = file_buf.clone();
        let mut affected_indexes : Vec<u32> = Vec::new();
        for i in 0..new_buf.len() {
            if new_buf.len() > 7 && i < new_buf.len() - 7 {
                if new_buf[i + 1] == new_buf[i] &&
                new_buf[i + 2] == new_buf[i] &&
                new_buf[i + 3] == new_buf[i] && 
                new_buf[i + 4] == new_buf[i] && 
                new_buf[i + 5] == new_buf[i] && 
                new_buf[i + 6] == new_buf[i] {
                    let mut cntr : u8 = 1;
                    while i + 1 < new_buf.len() && cntr < 255 && new_buf[i + 1] == new_buf[i] {
                        new_buf.remove(i + 1);
                        cntr += 1;
                    }
                    affected_indexes.push(i as u32);
                    new_buf.insert(i, cntr);
                }
            }
        }
        for i in 0..affected_indexes.len() {
            let bruh = affected_indexes[i].to_ne_bytes();
            for j in bruh {
                new_buf.insert(0, j);
            }
        }
        let num = (affected_indexes.len() as u32).to_ne_bytes();
        for thing in num {
            new_buf.insert(0, thing);
        }
        writefile.write_all(&new_buf);
    } else {
        let mut new_file_name = String::new();
        for i in 0..(file_name.clone().unwrap().len() - 12) {
            new_file_name.push(file_name.clone().unwrap().chars().nth(i).unwrap());
        }
        let mut writefile = File::create(new_file_name + ".uncrispied").expect("Making writefile failed");
        let mut file_buf : Vec<u8> = Vec::new();
        let file_length = readfile.read_to_end(&mut file_buf).expect("Reading into buffer in decompression mode didn't work");
        print_u8_vec(&file_buf);

    }

}

fn print_u8_vec(inp : &Vec<u8>) {
    println!();
    print!("{{");
    for i in inp {
        print!("{}, ", i);
    }
    print!("}}");
}
