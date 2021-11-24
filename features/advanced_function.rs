fn func_exit() -> bool {
  println!("hi");
  true
}

fn locate_func() -> fn() -> bool {
  func_exit
}

fn main() {
  let f = locate_func();
  f();
}