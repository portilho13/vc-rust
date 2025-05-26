use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

struct IVC {
    pub data: Vec<usize>,
    pub width: i32,
    pub height: i32,
    pub levels: i32,
    pub bytesperline: i32,
}

impl IVC {
    pub fn new(data: Vec<usize>, width: i32, height: i32, levels: i32, bytesperline: i32) -> Self {
        Self {
            data,
            width,
            height,
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
        if r.read_exact(&mut byte).is_err() {
            break; //EOF
        }

        let b = byte[0];

        if b.is_ascii_whitespace() {
            continue;
        }

        if b == b'#' {
            r.read_until(b'\n', &mut Vec::new())?;
        }

        buf.clear();
        buf.push(b);

        loop {
            match r.read_exact(&mut byte) {
                Ok(()) => {
                    let b = byte[0];
                    if b.is_ascii_whitespace() || b == b'#' {
                       if b == b'#' {
                           r.read_until(b'\n', &mut Vec::new())?;
                       }
                        break;
                    }
                    buf.push(b);
                }
                Err(_) => break,
            }
        }

        tk.push(String::from_utf8(buf.clone()).unwrap());

        if tk.len() == 4 {
            break;
        }
    }
    Ok(tk)
}

pub fn bit_to_uchar(databit: &[u8], datauchar: &mut [u8], width: i32, height: i32) {
    let x: i32;
    let y: i32;
    let mut countbits: i32;

    let p: &[u8] = databit;
    let mut p_idx = 0;

    countbits = 1;
    for y in 0..height {
        for x in 0..width {
            let pos = y * width + x;

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