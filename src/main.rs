use vc::{vc_convert_to_grayscale, vc_write_image};


mod vc;
fn main() -> std::io::Result<()> {
    let image = vc::vc_read_image("airplane.ppm");
    let gray = vc_convert_to_grayscale(&image).unwrap();
    vc_write_image("teste.ppm", gray).unwrap();
    Ok(())
}

