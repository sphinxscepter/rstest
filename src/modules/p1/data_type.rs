pub fn data_type_sty() {
  const a_length: usize = 5;
  let mut a: [i32; a_length] = [0; 5];
  for x in (0..5) {
    a[x] = x as i32;
  }
  for x in &a[1..] {
    println!("{}", x);
  }
}