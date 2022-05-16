
fn main() {

    // let a: i64 = 34;
    // let b = &a as *const i64;
    // let c: usize = unsafe {
    //     std::mem::transmute(b)
    // };

    // println!("a: {}, b: {} {:p}...{} 0x{:x}", a, b as usize,b, c, c);

    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}