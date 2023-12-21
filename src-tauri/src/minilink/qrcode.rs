use crate::Result;
use super::validation;

pub fn get_qrcode(link: String) -> Result<String> {
    validation::validate_link(&link)?;

    let size = 200;
    let image_src = format!("https://api.qrserver.com/v1/create-qr-code/?size={}x{}&data={}", size, size, link);
    
    Ok(image_src)
}