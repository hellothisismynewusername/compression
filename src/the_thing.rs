use std::io::{ErrorKind, Read, Write};
use std::fs::File;
use std::usize;

pub enum Bad {
    Nothing,
    TooLarge,
    Error(ErrorKind)
}

pub fn ball(mut files : Vec<File>) -> Result<Vec<u8>, Bad> {
    let files_num = files.iter().count();
    let byte_amount = files.iter().fold(0, |accum : u64, x| {
        accum + x.bytes().count() as u64
    });
    if byte_amount > u32::MAX as u64 {
        Err(Bad::TooLarge) 
    } else if byte_amount == 0 {
        Err(Bad::Nothing)
    } else {
        let mut out : Vec<u8> = Vec::new();

        let intro = files_num.to_le_bytes();
        for byte in intro.iter() {
            out.push(*byte);
        }

        //shove in the spots for the indices before concatting files bytes
        for i in 0..(files_num * 4) {
            out.push(0u8);
        }

        //concatting files bytes and determining their addresses
        let mut indices: Vec<usize> = Vec::new();
        for file in files.iter_mut() {

            indices.push(out.len() + (files_num * 4));  //don't have to add 4 b/c already added 4 bytes at the start before this

            let mut byte_vec = Vec::new();
            if file.read_to_end(&mut byte_vec).is_err() {
                return Err(Bad::Error(ErrorKind::Interrupted));
            }
            for byte in byte_vec.iter() {
                out.push(*byte);
            }

        }

        //addresses replacing the placeholder bytes
        for (i, ind) in indices.iter().enumerate() {
            let ind_in_bytes = ind.to_le_bytes();
            for (j, byte) in ind_in_bytes.iter().enumerate() {
                out[4 + (i * 4) + j] = *byte;
            }
        }

        Ok(out)
    }
}

pub fn compress(readfile : &mut File, file_name : &str, print : bool) -> Result<(), std::io::Error> {
    if print { println!("Reading file"); }

    let mut writefile = File::create(file_name.to_string() + ".crispyfries")?;
    let mut file_buf : Vec<u8> = Vec::new();
    readfile.read_to_end(&mut file_buf)?;

    if print { println!("Part 1"); }

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
            if print && i % 1000 == 0 {
                println!("{}%", (((i as f64 / file_buf.len() as f64) * 100000.) as u64) as f64 / 1000.);
            } 
            i += 1;
        }
    }
    if print { println!("Part 2"); }
    
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
    if print { println!("Writing file"); }

    writefile.write_all(&file_buf)
}