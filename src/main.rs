use std::{fs, io};

mod archiver;
mod un_archiver;
mod utils;

use archiver::Format;
use archiver::add_to_archive;

use std::path::{Path, PathBuf};

use crate::un_archiver::un_archive;
use crate::utils::get_files;

fn main() -> io::Result<()> {
    let mut files_arr: Vec<PathBuf> = Vec::new();
    let _ = get_files(Path::new("./sample"), &mut files_arr);

    let mut assinged_files: Vec<Format> = Vec::new();

    for path in files_arr {
        let filename = path.to_string_lossy().to_string();
        let content = fs::read(&path).unwrap();
        assinged_files.push(Format { filename, content });
    }
    add_to_archive(&assinged_files);
    let _ = un_archive("./ar.arc".to_string());
    Ok(())
}
