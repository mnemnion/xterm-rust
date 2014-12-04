//! #Xterm Library for Rust
//!
//! Sometimes you just wanna XTerm....

#![feature(globs)]
#![feature(macro_rules)]

pub use self::xvec::*;
pub use self::color::Colors;

mod escs;
pub mod xvec;
pub mod color;

pub mod nav;
