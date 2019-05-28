#![warn(unused)]

use crc::{crc32, Hasher32};

pub fn crc32fast_buffer(buf: &[u8]) -> u32 {
    // This is the initial value from bzlib
    let mut digest = crc32::Digest::new_custom(crc32::IEEE, !0u32, !0u32, crc::CalcType::Normal);
    digest.write(buf);
    digest.sum32()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_hashes() {
        let buf1 = "";
        let buf2 = " ";
        let buf3 = "hello world";

        let buf4 =
            concat!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, ",
                    "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ",
                    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ",
                    "ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit ",
                    "in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur ",
                    "sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt ",
                    "mollit anim id est laborum.");

        assert_eq!(crc32fast_buffer(buf1.as_bytes()), 0x00000000);
        assert_eq!(crc32fast_buffer(buf2.as_bytes()), 0x29d4f6ab);
        assert_eq!(crc32fast_buffer(buf3.as_bytes()), 0x44f71378);
        assert_eq!(crc32fast_buffer(buf4.as_bytes()), 0xd31de6c9);
    }
}
