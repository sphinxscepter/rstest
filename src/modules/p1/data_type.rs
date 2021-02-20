pub fn data_type_sty() {
  const A_LEN: usize = 5;
  let mut a: [i32; A_LEN] = [0; 5];
  for x in 0..A_LEN {
    a[x] = x as i32;
  }
  for x in &a[1..3] {
    println!("{}", x);
  }
}