fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn main() {
    let x = 5i;
    let y = 10i;

    println!("{}", match cmp(x, y) {
        Less    => "less",
        Greater => "greater",
        Equal   => "equal",
    });
}