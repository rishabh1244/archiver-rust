use std::path::Path;
use std::{fs, io};

pub const MAGIC: &[u8] = b"THISARCHIVE";

pub fn un_archive(dot_arc: String) -> io::Result<()> {
    let byte_data = fs::read(dot_arc)?;

    if byte_data.len() < MAGIC.len() {
        println!("Invalid Filetype");
        return Ok(());
    }
    let mut i = MAGIC.len(); // skip magic

    while i < byte_data.len() - 1 {
        let name_len = u16::from_le_bytes(byte_data[i..i + 2].try_into().unwrap()) as usize;
        i += 2;

        let file_size = u64::from_le_bytes(byte_data[i..i + 8].try_into().unwrap()) as usize;
        i += 8;

        let name_bytes = &byte_data[i..i + name_len];
        i += name_len;

        let name = std::str::from_utf8(name_bytes).unwrap();
        println!("name_len = {}", name_len);
        println!("name = {}", name);

        let content = byte_data[i..i + file_size].to_vec();
        i += file_size;

        println!("content bytes = {}", content.len());
        // find the size of the rest of file

        let path = Path::new(name);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }
        fs::write(path, content);
    }

    Ok(())
}

/*
00000000: 54 48 49 53 41 52 43 48 49 56 45 12 00 02 00 00  |THISARCHIVE.....|
00000010: 00 00 00 00 00 2E 2F 73 61 6D 70 6C 65 2F 66 69  |....../sample/fi|
00000020: 6C 65 31 2E 74 78 74 31 0A 11 00 02 00 00 00 00  |le1.txt1........|
00000030: 00 00 00 2E 2F 73 61 6D 70 6C 65 2F 66 69 6C 65  |..../sample/file|
00000040: 2E 74 78 74 30 0A 1C 00 02 00 00 00 00 00 00 00  |.txt0...........|
00000050: 2E 2F 73 61 6D 70 6C 65 2F 73 75 62 46 6F 6C 64  |./sample/subFold|
00000060: 65 72 2F 66 69 6C 65 32 2E 74 78 74 32 0A        |er/file2.txt2.|
*/
