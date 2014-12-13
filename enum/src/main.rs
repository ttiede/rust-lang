fn cmp(a: int, b: int) -> Ordering {
    if a == b { Equal }
    else if a < b { Less }
    else { Greater }

}

enum two_values {
    Value_one(int),
    Value_two(int),
}

fn main() {
    let x = 10i;
    let y = 10i;

    let ordering = cmp(x, y);

    println!("ordering: {}", ordering);

    if ordering == Less {
        println!("less");
    } else if ordering == Greater {
        println!("greater");
    } else if ordering == Equal {
        println!("equal");
    }

    let x = two_values::Value_one(5);
    let y = two_values::Value_two(10);
    println!("sum of values is: {}", x);

}