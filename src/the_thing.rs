use core::num;
use std::io::{Error, ErrorKind, Read, Write};
use std::fs::{File, OpenOptions};
use std::usize;

pub enum Bad {
    Nothing,
    TooLarge,
    IOError(Error),
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

            indices.push(out.len());  //don't have to add anythign b/c intro 4 bytes and placeholder 0s are in already

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

pub fn unball_and_write(ball : Vec<u8>, file_name : &str, print : bool) -> Result<(), std::io::Error> {
    let mut tmp_arr : [u8; 4] = [0; 4];
    let tmp_vec = ball.iter().enumerate().filter(|(i, _)| *i < 4).map(|(_, x)| *x).collect::<Vec<u8>>();
    for (i, x) in tmp_vec.iter().enumerate() {
        tmp_arr[i] = *x;
    }
    let num_files = u32::from_le_bytes(tmp_arr);

    let mut indices : Vec<u32> = Vec::new();
    for i in 0..num_files {
        let mut tmp_arr : [u8; 4] = [0; 4];
        let tmp_vec = ball.iter().enumerate().filter(|(index, x)| {
            *index > 4 + (i as usize * 4) && *index < 8 + (i as usize * 4)
        }).map(|(_, x)| *x).collect::<Vec<u8>>();
        for (ind, x) in tmp_vec.iter().enumerate() {
            tmp_arr[ind] = *x;
        }

        indices.push(u32::from_le_bytes(tmp_arr));
    }

    for i in 0..(num_files as usize) {
        let mut byte_vec : Vec<u8> = Vec::new();
        byte_vec = ball.iter().enumerate().filter(|(index, _)| {
            *index > indices[i] as usize && (*index < ball.len() || *index < indices[i + 1] as usize)
        }).map(|(_, x)| *x).collect();

        let mut writefile = File::create(format!("uncrispied file {}", i))?;
        writefile.write_all(&mut byte_vec);
    }

    // let mut i : u32 = 0;
    // let mut curr_file_num = 0;
    // while i < ball.len() as u32 {

    //     if indices[curr_file_num] == i {

    //         if curr_file_num > 0 { //start of a file has been reached but it's not the start of the first file
                
    //         }

    //         curr_file_num += 1;
    //     }

    //     i += 1;
    // }
    


    Ok(())
}

pub fn compress_and_write(readfile : &mut File, file_name : &str, print : bool) -> Result<(), std::io::Error> {
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

pub fn decompress(file_name : &str) -> Result<Vec<u8>, Bad> {
    // let mut new_file_name = String::new();
    // if (file_name.len() as i32) - 12 < 0 {
    //     new_file_name = String::from("out");
    // } else {
    //     for i in 0..(file_name.clone().unwrap().len() - 12) {
    //         new_file_name.push(file_name.clone().unwrap().chars().nth(i).unwrap());
    //     }
    // }

    let readfile_res = File::open(file_name);
    if readfile_res.is_err() {
        Err(Bad::IOError(readfile_res.err().unwrap()))
    } else {

        let mut readfile = readfile_res.ok().unwrap();

        let mut file_buf : Vec<u8> = Vec::new();
        readfile.read_to_end(&mut file_buf).expect("Reading into buffer in decompression mode didn't work");

        if (&readfile).bytes().count() > u32::MAX as usize {
            Err(Bad::TooLarge)
        } else if readfile.bytes().count() == 0 {
            Err(Bad::Nothing)
        } else {
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

            Ok(out)
        }
    }
}