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