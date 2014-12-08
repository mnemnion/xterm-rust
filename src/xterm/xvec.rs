use std::fmt;
use std::str::CowString;
use super::escs::{ANSI_D};
use super::nav::{save_cursor,
                 restore_cursor,
                 jump_string,
                 Point,
                 Frame};

/// XString: these are the valid types of string for xterm markup.
///
/// This will eventually be hidden.
pub enum XString<'b> {Esc(CowString<'b>), Jump(CowString<'b>), Text(CowString<'b>)}

impl<'b> fmt::Show for XString<'b> {
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
pub struct XVec<'b> {
   pub v: Vec<XString<'b>>
}

impl<'b> XVec<'b> {
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

/*
    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        for q in self.v.iter() {
            match *q {
                XString::Esc(ref q) => s = s.push_str(),
                XString::Jump(ref q) => s = s.push_str(q),
                XString::Text(ref q) => s = s.push_str(q),
            }
        }
        s
    }
*/
}
/*
impl<'b> fmt::Show for XVec<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}
*/
pub fn print_x ( xstr: XString ) -> () {
    match xstr {
        XString::Esc(s)  => println!("{}", s),
        XString::Jump(s) => println!("{}", s),
        XString::Text(s) => println!("{}", s),
    }
}

pub fn make_jump<'b> (pt: Point) -> XString<'b> {
    XString::Jump(jump_string(pt).into_cow())
}

pub fn line_split<'b> (s: &'b str) -> XVec<'b> {
    //! splits a line
    let mut x_vec  = XVec { v: vec![]};
    for line in s.split('\n') {
        x_vec.v.push(XString::Text(line.into_cow()));
    };
    x_vec
}
