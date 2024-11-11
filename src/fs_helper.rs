use actix_web::web::BytesMut;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn save_file(path: &Path, content: BytesMut) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&content)?;
    Ok(())
}
