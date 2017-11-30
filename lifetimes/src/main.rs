fn main() {
  let string1 = String::from("abcd");
  let result;

  {
    // will work
    let string2 = "xyz";
    result = longest(string1.as_str(), string2);

    // won't work
    // let string2 = String::from("xyz");
    // result = longest(string1.as_str(), string2.as_str());
  }

  println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}