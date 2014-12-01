use super::xvec::*;

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


// helper functions; unpublicize.

pub fn line_split (s: String) -> (XVec) {
    //! splits a line
    let mut x_vec  = XVec { v: vec![]};
    for line in s.as_slice().split('\n') {
        x_vec.v.push(XString::Text(line.to_string()));
    };
    x_vec
}
