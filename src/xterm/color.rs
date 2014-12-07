use super::xvec::*;
use super::escs::*;

#[allow(dead_code)]
pub enum Colors { Red, Blue, Green, Yellow, Magenta, Cyan, White, Default }


pub fn color_fg<'b> ( col: Colors ) -> XString<'b> {
    XString::Esc(match col {
        Colors::Red     => ANSI_R_F,
        Colors::Blue    => ANSI_B_F,
        Colors::Green   => ANSI_G_F,
        Colors::Yellow  => ANSI_Y_F,
        Colors::Magenta => ANSI_M_F,
        Colors::Cyan    => ANSI_C_F,
        Colors::White   => ANSI_W_F,
        Colors::Default => ANSI_D_F,
    }.into_cow())
}

pub fn color_bg<'b> ( col: Colors ) -> XString<'b> {
    XString::Esc(match col {
        Colors::Red     => ANSI_R_B,
        Colors::Blue    => ANSI_B_B,
        Colors::Green   => ANSI_G_B,
        Colors::Yellow  => ANSI_Y_B,
        Colors::Magenta => ANSI_M_B,
        Colors::Cyan    => ANSI_C_B,
        Colors::White   => ANSI_W_B,
        Colors::Default => ANSI_D_B,
    }.into_cow())

}
