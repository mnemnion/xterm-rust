pub const ANSI_R_F: &'static str = "\u001b[31m" ;
pub const ANSI_B_F: &'static str = "\u001b[34m" ;
pub const ANSI_G_F: &'static str = "\u001b[32m" ;
pub const ANSI_Y_F: &'static str = "\u001b[33m" ;
pub const ANSI_M_F: &'static str = "\u001b[35m" ;
pub const ANSI_C_F: &'static str = "\u001b[36m" ;
pub const ANSI_W_F: &'static str = "\u001b[37m" ;
pub const ANSI_D_F: &'static str = "\u001b[39m" ;

pub const ANSI_R_B: &'static str = "\u001b[41m" ;
pub const ANSI_B_B: &'static str = "\u001b[44m" ;
pub const ANSI_G_B: &'static str = "\u001b[42m" ;
pub const ANSI_Y_B: &'static str = "\u001b[43m" ;
pub const ANSI_M_B: &'static str = "\u001b[45m" ;
pub const ANSI_C_B: &'static str = "\u001b[46m" ;
pub const ANSI_W_B: &'static str = "\u001b[47m" ;
pub const ANSI_D_B: &'static str = "\u001b[49m" ;

pub const ANSI_D:   &'static str = "\u001b[0m" ;

pub const ANSI_SAVE:     &'static str = "\u001b7" ;
pub const ANSI_RESTORE:  &'static str = "\u001b8" ;
pub const ANSI_PAGE   :  &'static str = "\u001b[2J" ;

macro_rules! jump_fmt {
    () => ("\u001b[{};{}H")
}

pub fn jump_str (row: u16, col: u16) -> String {
    format!(jump_fmt!(), row, col)
}
