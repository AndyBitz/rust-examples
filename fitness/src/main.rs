use std::thread;
use std::time::Duration;

struct Cacher<T>
  where T: Fn(i32) -> i32
{
  calculation: T,
  value: Option<i32>
}

impl<T> Cacher<T>
  where T: Fn(i32) -> i32
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: i32) -> i32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn generate_workout(intensity: i32, random_number: i32) {
  let mut expensive_result = Cacher::new(|num: i32| -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_result.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}

fn main() {
  let simulated_user_specific_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specific_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
  let mut c = Cacher::new(|a| a);

  assert_eq!(c.value(1), 1);
  assert_eq!(c.value(2), 1);
  assert_eq!(c.value(3), 1);
}