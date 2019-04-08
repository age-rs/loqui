pub mod codec;
pub mod errors;
mod flags;
pub mod frames;
pub mod upgrade;

pub use self::flags::{is_compressed, make_flags, Flags};

pub const VERSION: u8 = 1;
