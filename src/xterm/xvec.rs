#![feature(globs)]
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
       print!( "\u001b[0m");
       self.print();
       print!("\u001b[0m");
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

fn save_cursor () -> () { print!("\u001b7") }
fn restore_cursor () -> () { print!("\u001b8") }
