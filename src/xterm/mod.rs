//! #Xterm Library for Rust
//!
//! Sometimes you just wanna XTerm....

pub use self::xvec::*;
pub use self::color::Colors;
pub mod xvec;
pub mod color;

pub mod nav;
mod escs;
