use std::io::{Read, Write};
use std::process::exit;
use std::fs::File;

mod the_thing;

fn main() {
    let mut file_name : Option<String> = None;
    let mut compression : bool = true;
    let mut verbose : bool = false;
    let mut arg_cntr = 0;
    for arg in std::env::args() {
        if arg_cntr == 1 {
            file_name = Some(arg.clone());
        }
        if arg_cntr == 2 || arg_cntr == 3 {
            if arg == "-d" {
                compression = false;
            }
            if arg == "-v" {
                verbose = true;
            }
        }
        arg_cntr += 1;
    }
    if arg_cntr > 3 || arg_cntr < 2 {
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
        if verbose { println!("Reading file"); }
        let mut writefile = File::create(file_name.unwrap() + ".crispyfries").expect("Making writefile failed");
        let mut file_buf : Vec<u8> = Vec::new();
        readfile.read_to_end(&mut file_buf).expect("Reading into buffer in compression mode didn't work");
        if verbose { println!("Part 1"); }
        let mut affected_indexes : Vec<u32> = Vec::new();
        {
        let mut i = 0;
            while i < file_buf.len() {
                if file_buf.len() > 7 && i < file_buf.len() - 7 {
                    if file_buf[i + 1] == file_buf[i] &&
                    file_buf[i + 2] == file_buf[i] &&
                    file_buf[i + 3] == file_buf[i] && 
                    file_buf[i + 4] == file_buf[i] && 
                    file_buf[i + 5] == file_buf[i] && 
                    file_buf[i + 6] == file_buf[i] {
                        let mut cntr : u8 = 1;
                        while i + 1 < file_buf.len() && cntr < 255 && file_buf[i + 1] == file_buf[i] {
                            file_buf.remove(i + 1);
                            cntr += 1;
                        }
                        affected_indexes.push(i as u32);
                        file_buf.insert(i, cntr);
                    }
                }
                if verbose && i % 1000 == 0 {
                    println!("{}%", (((i as f64 / file_buf.len() as f64) * 100000.) as u64) as f64 / 1000.);
                } 
                i += 1;
            }
        }
        if verbose { println!("Part 2"); }
        
        for i in 0..affected_indexes.len() {
            let bruh = affected_indexes[i].to_le_bytes();
            for j in bruh {
                file_buf.insert(0, j);
            }
        }
        let num = (affected_indexes.len() as u32).to_le_bytes();
        for thing in num {
            file_buf.insert(0, thing);
        }
        if verbose { println!("Writing file"); }

        writefile.write_all(&file_buf).expect("Couldn't write the file properly");
    } else {
        let mut new_file_name = String::new();
        if (file_name.clone().unwrap().len() as i32) - 12 < 0 {
            new_file_name = String::from("out");
        } else {
            for i in 0..(file_name.clone().unwrap().len() - 12) {
                new_file_name.push(file_name.clone().unwrap().chars().nth(i).unwrap());
            }
        }
        let mut writefile = File::create(new_file_name + ".uncrispied").expect("Making writefile failed");
        let mut file_buf : Vec<u8> = Vec::new();
        readfile.read_to_end(&mut file_buf).expect("Reading into buffer in decompression mode didn't work");

        let mut counter_buf : [u8; 4] = [0; 4];
        for i in 0..4 {
            counter_buf[i] = file_buf[i];
        }
        let counter : u32 = u32::from_be_bytes(counter_buf);
        let mut offset : usize = 4;
        let mut positions_buf : Vec<u8> = Vec::new();
        for _i in 0..counter {
            for j in 0..4 {
                positions_buf.push(file_buf[offset + j]);
            }
            offset += 4;
        }
        let mut positions : Vec<u32> = Vec::new();
        offset = 0;
        for _i in 0..counter {
            let mut items : [u8; 4] = [0; 4];
            for j in 0..4 {
                items[j] = positions_buf[offset + j];
            }
            offset += 4;
            positions.push(u32::from_be_bytes(items));
        }
        let ptr : usize= 4 + (4 * (counter as usize));
        let mut out : Vec<u8> = file_buf[ptr..file_buf.len()].to_vec();
        for pos in positions {
            let times : u8 = out[pos as usize] - 2;
            let byte : u8 = out[pos as usize + 1];
            out[pos as usize] = byte;
            for _i in 0..times {
                out.insert(pos as usize, byte);
            }
        }

        writefile.write_all(&out).expect("Couldn't write the file properly");
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