fn main() {
  let b = Box::new(5);
  println!("b = {}", b);
}

enum List {
  Cons(i32, List),
  Nil,
}