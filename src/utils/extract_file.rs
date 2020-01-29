use flate2::read::GzDecoder;
use tar::Archive;
use actix_web::web::Bytes;
use std::fs::{create_dir_all, File, remove_dir_all};
use std::path::Path;
use std::io::{Read, Write};

pub fn extract_file_and_inject(file: Bytes, path: &str) -> Result<(), ()> {
  let res = extract_file_and_inject_inner(file, path);
  match res {
    Err(e) => {
      let path = Path::new(path);
      if path.exists() {
        remove_dir_all(path);
      }
      Err(e)
    },
    Ok(v) => {
      Ok(v)
    }
  }
}

fn extract_file_and_inject_inner(file: Bytes, path: &str) -> Result<(), ()> {
  let data: &[u8] = &file;
  let tar = GzDecoder::new(data);
  let mut archive = Archive::new(tar);
  create_dir_all(path);
  archive.unpack(path).map_err(|_| ())?;
  let index_file_path = format!("{}/index.html", path);
  let mut src = File::open(&index_file_path).map_err(|_| ())?;
  let mut data = String::new();
  src.read_to_string(&mut data).map_err(|_| ())?;
  drop(src);
  let new_data = data.replace("</body>", r#"<script src="/template/bundle.js" async></script></body>"#);
  let mut dst = File::create(&index_file_path).map_err(|_| ())?;
  dst.write(new_data.as_bytes()).map_err(|_| ())?;
  Ok(())
}
