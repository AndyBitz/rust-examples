struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
  context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
  fn parse(&self) -> Result<(), &'s str> {
    Err(&self.context.0[1..])
  }
}

fn parse_context(context: Context) -> Result<(), &str> {
  Parser { context: &context }.parse()
}

fn main() {
  
}

/*
fn main() {
  let mut my_int = 10;
  println!("before change {:?}", my_int); // 10
  change_int(&mut my_int);
  println!("after change {:?}", my_int); // 30
}

fn change_int(x: &mut i32) {
  *x += 20;
}
*/