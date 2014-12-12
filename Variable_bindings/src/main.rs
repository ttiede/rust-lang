fn main() {
  let x: int;
  let y: f64;
  let sum: f64;
  
  
  x = 1;
  y = 2.0;
  sum = (x as int) as f64 + y;

  println!("The integer number is: {}", x);
  println!("The real number is: {}", y);
  println!("The sum numbers are: {}", sum);
}