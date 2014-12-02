#![feature(globs)]
use std::fmt;
use super::escs::{ANSI_D, ANSI_SAVE, ANSI_RESTORE};
use super::nav::{save_cursor,restore_cursor};

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
