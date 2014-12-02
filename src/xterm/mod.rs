#![feature(globs)]
#![feature(tuple_indexing)]
//use std::default::Default;

//! #Xterm Library for Rust
//!
//! Sometimes you just wanna XTerm....

use std::fmt;
pub use self::xvec::*;
pub use self::color::Colors;
pub mod xvec;
pub mod color;

pub mod nav;
mod escs;

// xvec.rs
// use color.rs

// end xvec.rs

// nav.rs
// end nav.rs

// colors.rs
// use xvec.rs (mutual dependency issue?)
