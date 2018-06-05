#[no_mangle]
pub extern "C" fn add_one(a: u32) -> u32 {
    a + 1
}

#[no_mangle]
pub extern "C" fn fib(n: u32) -> u32 {
    if n < 3 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
