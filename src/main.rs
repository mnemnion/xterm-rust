#![feature(globs)]
#![feature(tuple_indexing)]
//use std::default::Default;
use xterm::* ;
use xterm::XString::* ;

mod xterm {

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
pub fn page () -> () { print!("\u001b[2J")}

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

} // mod xterm


fn main() {
    page();
    println!("Hello, world!");
    let pt: Point = Point { row: 2, col: 25 };
    let x_vec : XVec = XVec { v: vec![Text("string".to_string()),
                              color_bg(Colors::Green),
                              make_jump(pt),
                              Text("green".to_string()),
                              color_fg(Colors::Default),]
  } ;
    let col_string: XString  = color_fg(Colors::Red) ;
    print!("{}",color_bg(Colors::Cyan)) ;
    print_x(col_string);
    //x_vec.print();
    x_vec.print_clean();
    print!("Oye! {}",color_fg(Colors::Default)) ;
    println!("");
}
