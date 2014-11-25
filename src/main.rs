mod xterm {

    pub fn hello() {
        println!("Hello Xterm!");
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
    print!("{}",msg);
    print!("{}",xterm::color_bg(xterm::Colors::Magenta)) ;
    xterm::hello();
    print!("{}",xterm::color_fg(xterm::Colors::Default)) ;
    println!("foo");
}
