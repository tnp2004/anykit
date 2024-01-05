use crate::{Result, validation};

pub fn get_qrcode(link: String) -> Result<String> {
    validation::validate_link(&link)?;

    let size = 200;
    let image_src = format!("https://api.qrserver.com/v1/create-qr-code/?size={}x{}&data={}", size, size, link);
    println!("{}", image_src);
    Ok(image_src)
}