use std::{fs, io};

mod archiver;
mod un_archiver;
mod utils;

use archiver::Format;
use archiver::add_to_archive;

use crate::un_archiver::un_archive;
use std::path::{Path, PathBuf};

use crate::utils::get_files;

fn main() -> io::Result<()> {
    println!("1 to archive 2 for un_archive");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    if choice == "1" {
        println!("insert file/folder name");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name)?;
        let file_name = file_name.trim();

        let mut files_arr: Vec<PathBuf> = Vec::new();
        get_files(Path::new(file_name), &mut files_arr)?; // don't ignore errors

        println!("Found {} files", files_arr.len()); // debug

        let mut assigned_files: Vec<Format> = Vec::new();

        for path in files_arr {
            let filename = path.to_string_lossy().to_string();
            let content = fs::read(&path)?;
            assigned_files.push(Format { filename, content });
        }

        add_to_archive(&assigned_files);
    } else {
        println!("insert .arc file");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name)?;
        let file_name = file_name.trim();

        un_archive(file_name.to_string())?;
    }
    Ok(())
}
