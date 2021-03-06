use super::escs::{ANSI_SAVE,ANSI_RESTORE, ANSI_PAGE, jump_str };


// belongs in escs.rs

// escs.rs

/// saves cursor at terminal level.
pub fn save_cursor () -> () { print!("{}",ANSI_SAVE) }
/// restores cursor at terminal level
pub fn restore_cursor () -> () { print!("{}",ANSI_RESTORE) }

#[deriving(PartialEq, Eq)]
/// represents a point in terminal space.
pub struct Point {
    pub row: u16,
    pub col: u16,
}

#[deriving(PartialEq, Eq)]
/// represents a rectangular region of terminal space.
pub struct Frame {
    pub tl: Point,
    pub br: Point,
}

impl Frame {
    /// returns a Frame with dimensions one less than original.
    ///
    /// **todo**: prevent a negative frame in the case of (0,0).
    /// this behavior would be very silly given the unsignedness of points.
    pub fn inside (&self) -> Frame {
        Frame { tl: Point { row: self.tl.row +1 , col: self.tl.col +1},
                br: Point { row: self.br.row -1 , col: self.br.col -1}}
    }
    /// returns a Frame of the same width and height, translated to pt.
    /// pt will be the top left, as is conventional.
    pub fn move_to (&self, pt: Point) -> Frame {
        let row_diff = self.br.row - self.tl.row;
        let col_diff = self.br.col - self.tl.col;
        Frame { tl: Point { row: pt.row, col: pt.col},
                br: Point { row: pt.row + row_diff,
                            col: pt.col + col_diff}}
    }
    /// tests whether or not a point is on top of a frame.
    /// needed, eventually, for intersection code.
    /// there will be many of these functions. I wrote one for practice.
    pub fn on_top (&self, pt: Point) -> bool {
        if (self.tl.row == pt.row) &
           (self.tl.col <= pt.col) &
           (self.br.col >= pt.col) { true }
        else { false }
    }
    // **todo** on_left, on_right, on_bottom
}

pub fn new_frame (anchor: Point, height: u16, width: u16 ) -> Frame {
    let fr= Frame { tl: anchor, br: Point { row: anchor.row + height,
                                col: anchor.col + width }};
    println!("Dimensions: {} {} {} {}", fr.tl.row, fr.tl.col, fr.br.row, fr.br.col);
    fr
}
    /// currently this just draws a box around a frame.
    ///
    /// eventually there will be several ways to do this.
    /// this function does not actually belong in nav.
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


    /// returns a string that will generate a jump to pt.
pub fn jump_string (pt: Point) -> String {
    jump_str(pt.row,pt.col)
}

    /// executes a jump.
pub fn jump(pt:Point) -> () { print!("{}",jump_string(pt)) }
pub fn page () -> () { print!("{}",ANSI_PAGE) }
pub fn cleanup () -> () { print!("\u001b[0m") }

#[test] fn new_frame_test () {
    let fr = Frame { tl: Point { row: 1,  col: 1  },
                        br: Point { row: 11, col: 11 } } ;
    let n_fr = new_frame( Point {row:1, col:1}, 10, 10) ;
    assert!(fr.eq(&n_fr));
}
#[test] fn move_to_test () {
    let fr = new_frame(Point{row:1,col:1},5,5).move_to(Point{row:7,col:7});
    let n_fr = new_frame(Point{row:7,col:7},5,5);
    assert!(fr.eq(&n_fr));
}

#[test] fn on_top_test () {
    let fr = new_frame(Point{row:1,col:1},5,5);
    assert!(fr.on_top(Point{row:1,col:3}));
    assert!(fr.on_top(Point{row:1,col:1}));
    assert!(fr.on_top(Point{row:1,col:5}));
    assert!(!fr.on_top(Point{row:1,col:7}));
    assert!(!fr.on_top(Point{row:5, col:3}));
}
