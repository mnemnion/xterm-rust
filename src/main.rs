#![feature(globs)]

use xterm::*;
use xterm::XString::*;
mod xterm;

fn main() {

    page();
    jump(Point{row: 1, col: 1});
    println!("Hello, world!");
    let pt: Point = Point { row: 2, col: 25 };
    let x_vec : XVec = XVec { v: vec![Text("wheredidIgo".to_string()),
                              color_bg(Colors::Green),
                              make_jump(pt),
                              Text("green".to_string()),
                              color_fg(Colors::Default),] };
    let col_string: XString  = color_fg(Colors::Red);
    print!("{}",color_bg(Colors::Cyan));
    print_x(col_string);
    print!("Hey there");
    x_vec.print_clean();
    jump(Point{row:5, col: 30});
    let nl_str = "a string\nwith plenty\nof newlines\nfor the purpose";
    let x_vec_b: XVec = line_split(nl_str.to_string());
    x_vec_b.print_clean();
    let frame = Frame { tl: Point { row: 5, col: 5}, br: Point { row: 9, col: 9} };
    draw_frame(frame);
    draw_frame(frame.inside());
    draw_frame(frame.move_to(Point{ row: 8, col: 30}));
    println!("\n");
    cleanup();
}
