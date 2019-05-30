#![warn(unused)]

mod crc32;

pub use crc32::{BZ2_finalise_crc, BZ2_initialise_crc, BZ2_update_crc};
