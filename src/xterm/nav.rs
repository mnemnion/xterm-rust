use super::escs::{ANSI_SAVE,ANSI_RESTORE};

pub fn save_cursor () -> () { print!("{}",ANSI_SAVE) }
pub fn restore_cursor () -> () { print!("{}",ANSI_RESTORE) }


pub struct Point {
    pub row: u16,
    pub col: u16,
}

pub struct Frame {
    pub tl: Point,
    pub br: Point,
}

impl Frame {
    pub fn inside (&self) -> Frame {
        Frame { tl: Point { row: self.tl.row +1 , col: self.tl.col +1},
                br: Point { row: self.br.row -1 , col: self.br.col -1}}
    }
    pub fn move_to (&self, pt: Point) -> Frame {
        let row_diff = self.br.row - self.tl.row;
        let col_diff = self.br.col - self.tl.col;
        Frame { tl: Point { row: pt.row, col: pt.col},
                br: Point { row: pt.row + row_diff,
                            col: pt.col + col_diff}}
    }
    pub fn on_top (&self, pt: Point) -> bool {
        if self.tl.row == pt.row { true }
        else { false }
    }
}

pub fn new_frame (anchor: Point, height: u16, width: u16 ) -> Frame {
    let fr= Frame { tl: anchor, br: Point { row: anchor.row + height,
                                col: anchor.col + width }};
    println!("Dimensions: {} {} {} {}", fr.tl.row, fr.tl.col, fr.br.row, fr.br.col);
    fr
}

pub fn draw_frame(f: Frame) -> () {
    save_cursor();
    jump(f.tl);
    print!("┏");
    for _ in range(f.tl.col, f.br.col-1) {
        print!("━");
    }
    print!("┓");
    for i in range(f.tl.row+1, f.br.row) {
        jump (Point {row: i, col: f.tl.col});
        print!("┃");
        jump (Point {row: i, col: f.br.col});
        print!("┃");
    }
    jump(Point{row: f.br.row, col: f.tl.col});
    print!("┗");
    for _ in range(f.tl.col, f.br.col-1) {
        print!("━");
    }
    print!("┛");
    restore_cursor();
}

pub fn jump_string(pt: Point) -> String {
    format!("\u001b[{};{}H",pt.row,pt.col)
}

pub fn jump(pt:Point) -> () { print!("{}",jump_string(pt)) }
pub fn page () -> () { print!("\u001b[2J") }
pub fn cleanup () -> () { print!("\u001b[0m") }

#[test]
fn new_frame_test () {
    let fr = Frame { tl: Point { row: 1,  col: 1  },
                        br: Point { row: 11, col: 11 } } ;
    let n_fr = new_frame( Point {row:1, col:1}, 10, 10) ;
    assert!(fr.tl.row == n_fr.tl.row);
    assert!(fr.tl.col == n_fr.tl.col);
    assert!(fr.br.row == n_fr.br.row);
    assert!(fr.br.col == n_fr.br.col);
}
