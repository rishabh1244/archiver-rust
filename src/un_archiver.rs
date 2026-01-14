use std::{fs, io};
pub const MAGIC: &[u8] = b"THISARCHIVE";

pub fn un_archive(dot_arc: String) -> io::Result<()> {
    let mut byte_data = fs::read(dot_arc)?;

    if byte_data.len() < MAGIC.len() {
        println!("Invalid Filetype");
        return Ok(());
    }
    let slice_data = &byte_data[0..MAGIC.len()];

    if slice_data != MAGIC {
        println!("Invalid Filetype");
        return Ok(());
    }
    // uncompress the files
    byte_data = byte_data[MAGIC.len()..byte_data.len()].to_vec();
    let _name_len = u16::from_le_bytes([byte_data[0], byte_data[1]]) as usize;
    println!("{}", _name_len);
    // find the size of the rest of file

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
