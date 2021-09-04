struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
  context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
  fn parse(&self) -> Result<&'s str, &'s str> {
    Ok(&self.context.0[1..])
  }
}

fn parse_context(context: Context) -> Result<&str, &str> {
  Parser { context: &context }.parse()
}

fn main() {
  // A string wrapped in a new struct with a lifetime specified
  // is put into another struct that takes the first struct as argument.
  // The new struct also has lifetimes specified.
  println!("Example 1:");
  {
    let txt = "hello world";
    let ctx = Context(&txt);
    let res = parse_context(ctx).unwrap();
    println!("Input: {}", txt);
    println!("Result: {}", res);
  }

  // The integer will be mutated by a function by using its reference.
  // Its lifetime will exceed that of the function that updates it.
  println!("\nExample 2:");
  {
    let mut my_int = 10;
    println!("Before change {:?}", my_int); // 10
    change_int(&mut my_int);
    println!("After change {:?}", my_int); // 30
  }
}

fn change_int(x: &mut i32) {
  *x += 20;
}
