mod vc;
fn main() -> std::io::Result<()> {
    let header = vc::netpbm_get_token("airplane.ppm")?;
    println!("{header:?}");
    Ok(())
}

