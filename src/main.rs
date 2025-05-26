mod vc;
fn main() -> std::io::Result<()> {
    let _ = vc::vc_read_image("airplane.ppm");
    Ok(())
}

