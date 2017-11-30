use std::thread;
// use std::sync::mpsc; // for channels
use std::sync::Mutex;

fn main() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}

// with channels
// fn main() {
//   let (tx, rx) = mpsc::channel();

//   let tx1 = mpsc::Sender::clone(&tx);

//   thread::spawn(move || {
//     let val = String::from("the first");
//     tx1.send(val).unwrap();
//   });

//   let handle = thread::spawn(move || {
//     let val = String::from("hi");
//     tx.send(val).unwrap();
//   });

//   for recv in rx {
//     println!("{:?}", recv);
//   }
// }
