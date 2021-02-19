use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn say_test() {
  let stdout = stdout();
  let message = String::from("hello ferris");
  let width = message.chars().count();
  let a = 123.0f32;
  // a = 1 as f32;
  println!("a = {0}", a);
  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();
}  