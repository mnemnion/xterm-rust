use std::fmt;
use super::escs::{ANSI_D};
use super::nav::{save_cursor,
                 restore_cursor,
                 jump_string,
                 Point,
                 Frame};

/// XString: these are the valid types of string for xterm markup.
///
/// This will eventually be hidden.
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

/// XVec: represents a composed string.
///
/// There are three types: Text contains the content, Esc is an escaped
/// sequence which is not a jump, and Jump is a esc[ which is.
pub struct XVec {
   pub v: Vec<XString>,
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
       print!("{}",ANSI_D);
       self.print();
       print!("{}",ANSI_D);
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

pub fn make_jump (pt: Point) -> XString {
    XString::Jump(jump_string(pt))
}


pub fn line_split (s: String) -> (XVec) {
    //! splits a line
    let mut x_vec  = XVec { v: vec![]};
    for line in s.as_slice().split('\n') {
        x_vec.v.push(XString::Text(line.to_string()));
    };
    x_vec
}
