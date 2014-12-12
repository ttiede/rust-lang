fn main() {
  struct Inches(int);
  let length = Inches(10);
  let length_other = Inches(12);

  let Inches(integer_length) = length;
  let Inches(integer_length_other) = length_other;
  
  println!("length is {} inches", integer_length);
  println!("length is {} inches", integer_length_other);

}