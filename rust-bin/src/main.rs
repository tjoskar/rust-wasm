fn main() {
  let result = fib(44);
  println!("Hello, world! {}", result);
}

fn fib(n: u32) -> u32 {
  if n < 3 {
    return 1;
  }
  return fib(n - 1) + fib(n - 2);
}
