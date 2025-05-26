use std::ffi::c_void;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::sync::mpsc::channel;

struct IVC {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub levels: i32,
    pub bytesperline: i32,
}

impl IVC {
    pub fn new(width: i32, height: i32, channels: i32, levels: i32) -> Self {
        let data_size = width * height * channels;
        let data: Vec<u8> = vec![0u8; data_size as usize];
        let bytesperline = width * channels;

        Self {
            data,
            width,
            height,
            channels,
            levels,
            bytesperline,
        }
    }
}


pub fn netpbm_get_token(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let mut r = BufReader::new(file);
    let mut tk: Vec<String> = Vec::new();
    let mut buf = Vec::<u8>::new();

    loop {
        let mut byte = [0u8; 1];

        // Skip whitespace and comments
        let b = loop {
            if r.read_exact(&mut byte).is_err() {
                return Ok(tk); // EOF
            }

            let b = byte[0];

            if b.is_ascii_whitespace() {
                continue;
            } else if b == b'#' {
                // Skip the rest of the comment line
                r.read_until(b'\n', &mut Vec::new())?;
                continue;
            } else {
                break b; // Start of a token
            }
        };

        // Start building a token
        buf.clear();
        buf.push(b);

        loop {
            match r.read_exact(&mut byte) {
                Ok(()) => {
                    let b = byte[0];

                    if b.is_ascii_whitespace() {
                        break;
                    }

                    if b == b'#' {
                        // Skip rest of comment and end token
                        r.read_until(b'\n', &mut Vec::new())?;
                        break;
                    }

                    buf.push(b);
                }
                Err(_) => break, // EOF
            }
        }

        let token = String::from_utf8_lossy(&buf).to_string();
        tk.push(token);
    }
}

fn uchar_to_bit(datauchar: &mut [u8], databit: &[u8], width: i32, height: i32) {
    let x: i32;
    let y: i32;
    let mut countbits: i32;
    let mut pos: i128;
    let mut counttotalbytes: i128;

    let mut p = databit.to_vec();
    let mut p_idx = 0;

    p[p_idx] = 0;
    counttotalbytes = 0;
    countbits = 1;



    for y in 0..height {
        for x in 0..width {
            pos = (y * width + x) as i128;

            if (countbits <= 8) {
                p[p_idx] |= ((datauchar[pos as usize] == 0) as u8) << (8 - countbits);

                countbits += 1;
            }
            if ((countbits > 8) || (x == width - 1)) {
                p_idx += 1;
                p[p_idx] = 0;
                countbits = 1;
                counttotalbytes += 1;
            }
        }
    }
}

fn bit_to_uchar(databit: &[u8], datauchar: &mut [u8], width: i32, height: i32) {
    let x: i32;
    let y: i32;
    let mut countbits: i32;

    let mut pos: i128;

    let p: &[u8] = databit;
    let mut p_idx = 0;

    countbits = 1;
    for y in 0..height {
        for x in 0..width {
            pos = (y * width + x) as i128;

            if (countbits <= 8) {
                let result = if (p[p_idx as usize] & (1 << (8 - countbits))) != 0 {
                    0
                } else {
                    1
                };

                datauchar[pos as usize] = result;
            }
            if ((countbits > 8) || (x == width - 1)) {
                p_idx += 1;
                countbits = 1;
            };
        }
    }
}

pub fn vc_read_image(file_name: &str) {

    let height: i32;
    let width: i32;
    let mut levels: i32 = 256;
    let mut channels: i32 = 1;

    let sizeofbinarydata: i64;

    let mut image: IVC;

    let _ = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    }; // Panic if file doesnt exist

    let file_content = netpbm_get_token(file_name).unwrap();
    let header = file_content.get(0..4).unwrap();
    println!("header: {:?}", header);

    if header[0] == "P4" {
        channels = 1;
        levels = 2;
    } else if header[0] == "P5" {
        channels = 1;
    } else if header[0] == "P6" {
        channels = 3;
    }

    width = header[1].parse().unwrap();
    height = header[2].parse().unwrap();

    if levels == 2 {
        image = IVC::new(width, height, channels, levels);
    } else {
        image = IVC::new(width, height, channels, levels);
        let data_tokens = &file_content[4..];
        let bytes: Vec<u8> = data_tokens
            .iter()
            .flat_map(|s| s.as_bytes())
            .copied()
            .collect();

        image.data = bytes;
    }
}