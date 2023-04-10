// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// 所有权转移了, 应该使用 &p

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 100, y: 200 };
    let y: Option<&Point> = Some(&p);

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
