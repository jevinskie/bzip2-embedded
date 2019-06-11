#![warn(unused)]

mod crc32;
mod rand_table;

pub use crc32::{BZ2_finalise_crc, BZ2_initialise_crc, BZ2_update_crc};
pub use rand_table::{BZ2_rand_init, BZ2_rand_mask, BZ2_rand_update_mask};
