#![feature(globs)]
#![feature(tuple_indexing)]
use std::default::Default;
use std::fmt;
use xterm::*;
use xterm::XString::* ;

mod xterm {
    use std::fmt;

    pub enum XString { Esc(String), Jump(String), Text(String) }

    impl fmt::Show for XString {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = String::new();
            let s = match self {
                &XString::Esc(ref q)  => q,
                &XString::Jump(ref q) => q,
                &XString::Text(ref q) => q,
            };
           write!(f,"{}",s)
        }
    }

    pub struct XVec {
        v: Vec<XString>,
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

    pub fn color_bg ( col: Colors ) -> String {
        match col {
            Colors::Red     => "\u001b[41m",
            Colors::Blue    => "\u001b[44m",
            Colors::Green   => "\u001b[42m",
            Colors::Yellow  => "\u001b[43m",
            Colors::Magenta => "\u001b[45m",
            Colors::Cyan    => "\u001b[46m",
            Colors::White   => "\u001b[47m",
            Colors::Default => "\u001b[0m",
        }.to_string()

    }
}


fn main() {
    println!("Hello, world!");
    let col_string: XString  = color_fg(Colors::Green) ;
    print!("{}",Esc(color_bg(Colors::Magenta))) ;
    print_x(col_string);
    print!("Oye! {}",color_fg(Colors::Default)) ;
    println!("");
}
