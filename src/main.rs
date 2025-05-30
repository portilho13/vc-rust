
mod vc;
fn main() -> std::io::Result<()> {
    let _ = vc::vc_read_image("airplane.ppm");
    //vc_save_image("teste.ppm", image).unwrap();
    Ok(())
}

