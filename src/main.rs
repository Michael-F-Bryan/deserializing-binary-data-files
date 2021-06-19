use std::{
    io::{Error, Read, Write},
    mem::{self, MaybeUninit},
};

fn main() {
    todo!();
}

#[derive(Debug)]
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
        // Create an uninitialized Speaker variable
        let mut speaker = MaybeUninit::<Speaker>::uninit();

        unsafe {
            // Get a slice which treats the `speaker` variable as a byte array
            let buffer: &mut [u8] = std::slice::from_raw_parts_mut(
                speaker.as_mut_ptr().cast(),
                mem::size_of::<Speaker>(),
            );

            // Read exactly that many bytes from the reader
            reader.read_exact(buffer)?;

            // Our `speaker` has now been initialized
            Ok(speaker.assume_init())
        }
    }
}
