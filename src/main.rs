#![feature(globs)]
use xterm::*;
use xterm::XString::* ;

mod xterm {

    pub fn hello() {
        println!("Hello Xterm!");
    }

    struct EscString(String);
    struct JuString(String);
    struct TString(String);

    pub enum XString { Esc(EscString), Jump(JuString), Text(TString) }

    pub fn print_X ( xstr: XString ) -> () {
        match xstr {
            XString::Esc(s)  => println!("{}", s.0),
            XString::Jump(s) => println!("{}", s.0),
            XString::Text(s) => println!("{}", s.0),
        }
    }

    pub enum Colors { Red, Blue, Green, Yellow, Magenta, Cyan, White, Default }

    pub fn color_fg ( col: Colors ) -> &'static str {
        match col {
            Colors::Red     => "\u001b[31m",
            Colors::Blue    => "\u001b[34m",
            Colors::Green   => "\u001b[32m",
            Colors::Yellow  => "\u001b[33m",
            Colors::Magenta => "\u001b[35m",
            Colors::Cyan    => "\u001b[36m",
            Colors::White   => "\u001b[37m",
            Colors::Default => "\u001b[0m",
        }
    }


    pub fn color_fg_xstr ( col: Colors ) -> EscString {
        let s = match col {
            Colors::Red     => "\u001b[31m",
            Colors::Blue    => "\u001b[34m",
            Colors::Green   => "\u001b[32m",
            Colors::Yellow  => "\u001b[33m",
            Colors::Magenta => "\u001b[35m",
            Colors::Cyan    => "\u001b[36m",
            Colors::White   => "\u001b[37m",
            Colors::Default => "\u001b[0m",
        };

        EscString(s.to_string())
    }

    pub fn color_bg ( col: Colors ) -> &'static str {
        match col {
            Colors::Red     => "\u001b[41m",
            Colors::Blue    => "\u001b[44m",
            Colors::Green   => "\u001b[42m",
            Colors::Yellow  => "\u001b[43m",
            Colors::Magenta => "\u001b[45m",
            Colors::Cyan    => "\u001b[46m",
            Colors::White   => "\u001b[47m",
            Colors::Default => "\u001b[0m",
        }

    }
}


fn main() {
    println!("Hello, world!");
    let msg: &'static str = xterm::color_fg(xterm::Colors::Green) ;
    let col_string: XString  = Esc(color_fg_xstr(xterm::Colors::Green)) ;
    print!("{}",msg);
    print!("{}",color_bg(Colors::Magenta)) ;
    print_X(col_string);
    hello();
    print!("{}",color_fg(Colors::Default)) ;
    println!("");
}
