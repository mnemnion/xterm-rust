#![feature(globs)]
#![feature(tuple_indexing)]
//use std::default::Default;

//! #Xterm Library for Rust
//!
//! Sometimes you just wanna XTerm....

use std::fmt;

pub enum XString { Esc(String), Jump(String), Text(String) }

impl fmt::Show for XString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            XString::Esc(ref q)  => q,
            XString::Jump(ref q) => q,
            XString::Text(ref q) => q,
        };
       write!(f,"{}",s)
    }
}

pub struct XVec {
   pub v: Vec<XString>,
}

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

impl XVec {
   pub fn print (&self) -> () {
      for q in  self.v.iter() {
          match *q {
              XString::Esc(ref q)  => print!("{}", q),
              XString::Jump(ref q) => print!("{}", q),
              XString::Text(ref q) => print!("{}", q),
          }
       }
    }
   pub fn print_clean (&self) -> () {
       save_cursor();
       print!("{}",color_fg(Colors::Default));
       self.print();
       print!("{}",color_fg(Colors::Default));
       restore_cursor();

   }
}

pub fn print_x ( xstr: XString ) -> () {
    match xstr {
        XString::Esc(s)  => println!("{}", s),
        XString::Jump(s) => println!("{}", s),
        XString::Text(s) => println!("{}", s),
    }
}

#[allow(dead_code)]
pub enum Colors { Red, Blue, Green, Yellow, Magenta, Cyan, White, Default }

fn save_cursor () -> () { print!("\u001b7") }
fn restore_cursor () -> () { print!("\u001b8") }

pub fn color_fg ( col: Colors ) -> XString {
    XString::Esc(match col {
        Colors::Red     => "\u001b[31m",
        Colors::Blue    => "\u001b[34m",
        Colors::Green   => "\u001b[32m",
        Colors::Yellow  => "\u001b[33m",
        Colors::Magenta => "\u001b[35m",
        Colors::Cyan    => "\u001b[36m",
        Colors::White   => "\u001b[37m",
        Colors::Default => "\u001b[0m",
    }.to_string())
}

pub fn color_bg ( col: Colors ) -> XString {
    XString::Esc(match col {
        Colors::Red     => "\u001b[41m",
        Colors::Blue    => "\u001b[44m",
        Colors::Green   => "\u001b[42m",
        Colors::Yellow  => "\u001b[43m",
        Colors::Magenta => "\u001b[45m",
        Colors::Cyan    => "\u001b[46m",
        Colors::White   => "\u001b[47m",
        Colors::Default => "\u001b[0m",
    }.to_string())

}

pub fn make_jump(pt: Point) -> XString {
    XString::Jump(format!("\u001b[{};{}H",pt.row,pt.col))
}

pub fn jump(pt:Point) -> () { print!("{}",make_jump(pt)) }
pub fn page () -> () { print!("\u001b[2J") }
pub fn cleanup () -> () { print!("\u001b[0m") }

// helper functions; unpublicize.

pub fn line_split (s: String) -> (XVec) {
    //! splits a line
    let mut x_vec  = XVec { v: vec![]};
    for line in s.as_slice().split('\n') {
        x_vec.v.push(XString::Text(line.to_string()));
    };
    x_vec
}



