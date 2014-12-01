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

// xvec.rs
// use color.rs

// end xvec.rs

// nav.rs
pub struct Point {
    pub row: u16,
    pub col: u16,
}

pub struct Frame {
    pub tl: Point,
    pub br: Point,
}

impl Frame {
    pub fn inside (&self) -> Frame {
        Frame { tl: Point { row: self.tl.row +1 , col: self.tl.col +1},
                br: Point { row: self.br.row -1 , col: self.br.row -1}}
    }
    pub fn move_to (&self, pt: Point) -> Frame {
        let row_diff = self.br.row - self.tl.row;
        let col_diff = self.br.col - self.tl.col;
        Frame { tl: Point { row: pt.row, col: pt.col},
                br: Point { row: pt.row + row_diff,
                            col: pt.col + col_diff}}
    }
}

pub fn draw_frame(f: Frame) -> () {
    jump(f.tl);
    print!("┏");
    for i in range(f.tl.col, f.br.col-1) {
        print!("━");
    }
    print!("┓");
    for i in range(f.tl.row+1, f.br.row) {
        jump (Point {row: i, col: f.tl.col});
        print!("┃");
        jump (Point {row: i, col: f.br.col});
        print!("┃");
    }
    jump(Point{row: f.br.row, col: f.tl.col});
    print!("┗");
    for i in range(f.tl.col, f.br.col-1) {
        print!("━");
    }
    print!("┛");
}

pub fn make_jump(pt: Point) -> XString {
    XString::Jump(format!("\u001b[{};{}H",pt.row,pt.col))
}

pub fn jump(pt:Point) -> () { print!("{}",make_jump(pt)) }
pub fn page () -> () { print!("\u001b[2J") }
pub fn cleanup () -> () { print!("\u001b[0m") }

// end nav.rs

// colors.rs
// use xvec.rs (mutual dependency issue?)
