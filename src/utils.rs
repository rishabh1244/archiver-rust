use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn hexdump(buf: &[u8]) {
    for (row, chunk) in buf.chunks(16).enumerate() {
        print!("{:08X}: ", row * 16);

        for b in chunk {
            print!("{:02X} ", b);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }

        print!(" |");
        for &b in chunk {
            let c = if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            };
            print!("{c}");
        }
        println!("|");
    }
}

pub fn get_files(dir: &Path, files_arr: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files_arr.push(path);
        } else if path.is_dir() {
            get_files(&path, files_arr)?;
        }
    }
    Ok(())
}
