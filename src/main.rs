use std::{
    fmt::{self, Debug, DebugStruct, Formatter},
    fs::File,
    io::{Error, Read, Write},
    mem,
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let filename = args.get(1).expect("Usage: ./main <filename>");

    let f = File::open(filename).unwrap();
    let speaker = Speaker::load(f).unwrap();

    println!("{:?}", speaker);
}

#[repr(C)]
pub struct Speaker {
    name: [[u8; 20]; 2],
    addr1: [u8; 40],
    addr2: [u8; 40],
    phone: [u8; 16],
    flags: u16,
}

impl Speaker {
    pub fn load(mut reader: impl Read) -> Result<Self, Error> {
        // Create a Speaker where all the fields are set to some sane default
        let mut speaker = Speaker {
            name: [[0; 20]; 2],
            addr1: [0; 40],
            addr2: [0; 40],
            phone: [0; 16],
            flags: 0,
        };

        // Safety: All the fields in a Speaker are valid for all possible bit
        // combinations.
        unsafe {
            // Get a slice which treats the `speaker` variable as a byte array
            let buffer: &mut [u8] = std::slice::from_raw_parts_mut(
                &mut speaker as *mut Speaker as *mut u8,
                mem::size_of::<Speaker>(),
            );

            // Read exactly that many bytes from the reader
            reader.read_exact(buffer)?;

            // Our `speaker` has now been updated with data from the reader.
            Ok(speaker)
        }
    }

    pub fn save(&self, mut writer: impl Write) -> Result<(), Error> {
        // Safety: it's always valid to cast something to an array of bytes.
        unsafe {
            let buffer = std::slice::from_raw_parts(
                self as *const Speaker as *const u8,
                mem::size_of::<Speaker>(),
            );

            writer.write_all(buffer)
        }
    }

    pub fn name(&self) -> Option<(&str, &str)> {
        let [first, last] = &self.name;
        let first = c_string(first)?;
        let last = c_string(last)?;

        Some((first, last))
    }

    pub fn address_line_1(&self) -> Option<&str> { c_string(&self.addr1) }

    pub fn address_line_2(&self) -> Option<&str> { c_string(&self.addr2) }

    pub fn phone_number(&self) -> Option<&str> { c_string(&self.phone) }
}

fn c_string(bytes: &[u8]) -> Option<&str> {
    let bytes_without_null = match bytes.iter().position(|&b| b == 0) {
        Some(ix) => &bytes[..ix],
        None => bytes,
    };

    std::str::from_utf8(bytes_without_null).ok()
}

impl Debug for Speaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Speaker {
            name: [first, last],
            addr1,
            addr2,
            phone,
            flags,
        } = self;

        let mut writer = f.debug_struct("Speaker");

        string_field(&mut writer, "first_name", first);
        string_field(&mut writer, "last_name", last);
        string_field(&mut writer, "addr1", addr1);
        string_field(&mut writer, "addr2", addr2);
        string_field(&mut writer, "phone", phone);
        writer.field("flags", &format_args!("{:#x}", flags));

        writer.finish()
    }
}

fn string_field(f: &mut DebugStruct<'_, '_>, name: &str, field: &[u8]) {
    match c_string(field) {
        Some(field) => {
            f.field(name, &field);
        },
        None => {
            f.field(name, &field);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const DEMO_DAT: [u8; mem::size_of::<Speaker>()] = [
        0x4a, 0x6f, 0x73, 0x65, 0x70, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x6c, 0x6f, 0x67,
        0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x31, 0x32, 0x33, 0x20, 0x46, 0x61, 0x6b, 0x65,
        0x20, 0x53, 0x74, 0x72, 0x65, 0x65, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x65, 0x77, 0x20,
        0x59, 0x6f, 0x72, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x30, 0x31, 0x31, 0x37,
        0x00, 0x00, 0x00, 0x00, 0x0f, 0xaa,
    ];

    #[test]
    fn empty_c_string() {
        let buffer = [];

        let got = c_string(&buffer).unwrap();

        assert_eq!(got, "");
    }

    #[test]
    fn c_string_full_of_nulls() {
        let buffer = [0; 42];

        let got = c_string(&buffer).unwrap();

        assert_eq!(got, "");
    }

    #[test]
    fn c_string_with_no_nulls() {
        let buffer = b"Hello, World!";

        let got = c_string(buffer).unwrap();

        assert_eq!(got, "Hello, World!");
    }

    #[test]
    fn c_string_with_internal_null() {
        let buffer = b"Hello,\0 World!";

        let got = c_string(buffer).unwrap();

        assert_eq!(got, "Hello,");
    }

    #[test]
    fn deserialize_joe_bloggs() {
        let reader = Cursor::new(&DEMO_DAT);

        let got = Speaker::load(reader).unwrap();

        assert_eq!(got.name().unwrap(), ("Joseph", "Blogs"));
        assert_eq!(got.address_line_1().unwrap(), "123 Fake Street");
        assert_eq!(got.address_line_2().unwrap(), "New York");
        assert_eq!(got.phone_number().unwrap(), "202-555-0117");
        assert_eq!(got.flags, 0xAA0F);
    }
}
