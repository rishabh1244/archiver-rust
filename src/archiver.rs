use std::fs;

pub const MAGIC: &[u8] = b"THISARCHIVE";

#[derive(Debug, Clone)]
pub struct Format {
    pub filename: String,
    pub content: Vec<u8>,
}
impl Format {
    // returns out vector of type u8 (one byte per index) containing the hex format of a specific
    // file
    fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let name_bytes = self.filename.as_bytes();
        /*
                 we are using u16 but the array is of vec<u8> type thats why we are doing
                 name_len.to_le_bytes() converting it to u8 format .

                so when converting the whole out vec to characters will only give us the u8 type feilds (1
                byte per character) and will not print out the numbers (more than 1 bytes ) unless specified.

                continue reading below
        */

        let name_len = name_bytes.len() as u16;
        let file_size = self.content.len() as u64;

        out.extend_from_slice(&name_len.to_le_bytes());
        out.extend_from_slice(&file_size.to_le_bytes());

        out.extend_from_slice(name_bytes);
        out.extend_from_slice(&self.content);

        out
    }
}
// takes in an array of files and runs serialize for all of them and write it as an archive
pub fn add_to_archive(files_array: &[Format]) {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(MAGIC);

    for file in files_array {
        out.extend_from_slice(&file.serialize());
    }

    /*
        here printing u8 vector:-
            ascii characters lie in the range 0-127 where 0-31 are control characters
    */
    // hexdump(&out);

    /*
       for &b in &out {
            let c = if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            };
            print!("{}", c);
        }
    */
    let _ = fs::write("./ar.arc", out);
}
