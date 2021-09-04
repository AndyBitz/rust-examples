fn main() {
  let stack = 3;
  let stack2 = 4;
  let heap = Box::new(5);
  let heap2 = Box::new(7);

  println!(" stack @ {:p} with {}", &stack, stack);
  println!("stack2 @ {:p} with {}", &stack2, stack2);
  println!("  heap @ {:p} with {}", heap, heap);
  println!(" heap2 @ {:p} with {}", heap2, heap2);
}
