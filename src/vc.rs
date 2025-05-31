use std::fs::File;
use std::io::{self, Result, Seek, Write};
use std::io::{BufRead, BufReader, Read};

pub struct IVC {
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


pub fn netpbm_get_token(file_name: &str) -> io::Result<(Vec<String>, usize)> {
    let file = File::open(file_name)?;
    let mut r = BufReader::new(file);
    let mut tk: Vec<String> = Vec::new();
    let mut buf = Vec::<u8>::new();

    let mut bytecount = 0;

    loop {
        let mut byte = [0u8; 1];

        // Skip whitespace and comments
        let b = loop {
            if r.read_exact(&mut byte).is_err() {
                return Ok((tk, bytecount)); // EOF
            }
            bytecount += 1;

            let b = byte[0];

            if b.is_ascii_whitespace() {
                continue;
            } else if b == b'#' {
                // Skip the rest of the comment line
                let mut dummy = Vec::new();
                let bytes_read = r.read_until(b'\n', &mut dummy)?;
                bytecount += bytes_read;
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
                    bytecount += 1;
                    let b = byte[0];

                    if b.is_ascii_whitespace() {
                        break;
                    }

                    if b == b'#' {
                        let mut dummy = Vec::new();
                        let bytes_read = r.read_until(b'\n', &mut dummy)?;
                        bytecount += bytes_read;
                        break;
                    }

                    buf.push(b);
                }
                Err(_) => break, // EOF
            }
        }

        let token = String::from_utf8_lossy(&buf).to_string();
        tk.push(token);

        if tk.len() == 4 {
            break;
        }
    }

    Ok((tk, bytecount))
}




pub fn vc_read_image(file_name: &str) -> IVC {

    let height: i32;
    let width: i32;
    let mut levels: i32 = 256;
    let mut channels: i32 = 1;

    let mut image: IVC;

    let mut file = File::open(file_name).unwrap(); // Panic if file doesnt exist


    let (file_content, bytecount) = netpbm_get_token(file_name).unwrap();
    let header = file_content.get(0..4).unwrap();

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

    }

    file.seek(io::SeekFrom::Start(bytecount as u64)).unwrap();

    let mut content:  Vec<u8> = Vec::new();

    file.read_to_end(&mut content).unwrap();

    image.data = content;

    image
}


pub fn vc_write_image(file_name: &str, mut image: IVC) -> Result<()> {

    let header: String;

    if image.levels != 2 {
        let file_type = if image.channels == 1 {
            "P5"
        } else {
            "P6"
        };
        
        header = format!("{} \n{} {} \n{}\n", file_type, image.width, image.height, image.levels - 1);
    } else {
        header = format!("{} \n{} {}\n", "P4", image.width, image.height)
    }

    let mut header_as_bytes = header.as_bytes().to_vec();

    let mut file = File::create(file_name).unwrap();

    let mut content: Vec<u8> = Vec::new();

    content.append(&mut header_as_bytes); // Append Header

    content.append(&mut image.data); // Append Image Data

    file.write_all(&content).unwrap();
    Ok(())
}


pub fn vc_convert_to_grayscale(src: &IVC) -> Result<IVC> {
    let mut dst = IVC::new(src.width, src.height, 1, src.levels);

    for y in 0..src.height {
        for x in 0..src.width {
            let pos_src = y * src.bytesperline + x * src.channels;

            let r = src.data[pos_src as usize];
            let g = src.data[(pos_src + 1) as usize];
            let b = src.data[(pos_src + 2) as usize];

            let gray = (r as f64 * 0.299) + (g as f64 * 0.587) + (b as f64 * 0.114);
            let gray_u8 = gray.round() as u8;

            let pos_dst = y * dst.bytesperline + x;
            dst.data[pos_dst as usize] = gray_u8;
        }
    }

    Ok(dst)
}
