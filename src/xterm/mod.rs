//! #Xterm Library for Rust
//!
//! Sometimes you just wanna XTerm....

pub use self::xvec::*;
pub use self::color::Colors;

mod escs;
pub mod xvec;
pub mod color;

pub mod nav;

#[macro_escape]
macro_rules! jump_fmt {
    () => ("\u001b[{};{}H")
}
