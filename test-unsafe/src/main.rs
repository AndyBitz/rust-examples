
/*
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (slice::from_raw_parts_mut(ptr, mid),
     slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
  }
}

fn main() {
  let mut my_stuff = [1,2,3,4,5,6,7,8];

  let (my_one, my_two) = split_at_mut(&mut my_stuff, 4);
  println!("one {:?}", my_one);
  println!("two {:?}", my_two);
}
*/



/*
fn main() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  // *r2 = 6; // not possible

  unsafe {
    *r2 = 6; // possible

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}
*/