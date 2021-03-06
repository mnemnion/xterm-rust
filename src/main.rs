#![feature(globs)]
#![feature(macro_rules)]

use xterm::*;
use xterm::XString::*;
use xterm::color::{color_fg, color_bg};
use xterm::nav::*;
mod xterm;

fn old_main() {
    page();
    jump(Point{row: 1, col: 1});
    println!("Hello, world!");
    let pt: Point = Point { row: 2, col: 25 };
    let x_vec : XVec = XVec { v: vec![Text("wheredidIgo".into_cow()),
                              color_bg(Colors::Green),
                              make_jump(pt),
                              Text("green".into_cow()),
                              color_fg(Colors::Default),] };
    let col_string: XString  = color_fg(Colors::Red);
    print!("{}",color_bg(Colors::Cyan));
    print_x(col_string);
    print!("Hey there");
    x_vec.print_clean();
    jump(Point{row:5, col: 30});
    let nl_str = "a string\nwith plenty\nof newlines\nfor the purpose";
    let x_vec_b: XVec = line_split(nl_str);
    x_vec_b.print_clean();
    let frame = Frame { tl: Point { row: 5, col: 5}, br: Point { row: 9, col: 9} };
    draw_frame(frame);
    draw_frame(frame.inside());
    draw_frame(frame.move_to(Point{ row: 8, col: 30}));
    println!("\n");
    cleanup();
}

fn main() {
    old_main(); // write some tests already
    page();
    jump(Point{row: 1, col: 1});
    let frame = new_frame ( Point { row: 5, col: 50}, 6, 10 );
    draw_frame(frame);
    draw_frame(frame.inside());
    let other_frame = Frame { tl: Point { row: 5, col: 5}, br: Point { row: 9, col: 9} };
    let inside = frame.inside();
  //  println!("inside: {} {} {} {}", inside.tl.row, inside.tl.col, inside.br.row, inside.br.col);
    draw_frame(other_frame);
    draw_frame(other_frame.inside());
    draw_frame(other_frame.move_to(Point{ row: 8, col: 30}).inside());
    cleanup();
}
