fn main() {
  let x = (1i, "hello");
  println!("x is {}", x);

  let x: (int, &str) = (1, "hello");
  println!("x is {}", x);

  let x: (int, &str) = (1, "hello");
  println!("x is {}", x);
  
  let (x, y, z) = (1i, 2i, 3i);
  println!("x is {}", x);

  let x = (1i, 2i, 3i);
  let y = (2i, 2i, 4i);

  println!("x is {}", x);

  if x == y {
    println!("yes");
  } else {
    println!("no");
  }

 let (x, y) = next_two(5i);
 println!("x, y = {}, {}", x, y);

}

fn next_two(x: int) -> (int, int) { (x + 1i, x + 2i) }