pub const title: &str = "Japanese";
mod address;
mod phone_number;
mod cell_phone;
mod name;
pub use self::address::*;
pub use self::phone_number::*;
pub use self::cell_phone::*;
pub use self::name::*;
