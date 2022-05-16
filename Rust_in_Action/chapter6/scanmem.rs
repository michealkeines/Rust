////fn main() {
    // let mut not_zero = 0;

    // for i in 1..10_000 {
    //     println!("{}",i);
    //     let ptr = i as *const usize;
    //     let val = unsafe { *ptr };

    //     if val != 0 {
    //         not_zero  += 1;
    //     }
    // }

    // println!("number of Non-zero val in current process mem: {}", not_zero);

    static GLOBAL: i32 = 1999;

    fn noop() -> *const u32 {
        let noop_local: u32 = 2;
        let temp = &noop_local as *const u32;
        unsafe {
            println!("{:032b}", *temp);
        }
        
        &noop_local as *const u32
        
    }

    fn main() {
        let local_str = "test";
        let local_int = 1234;
        let boxed_str = Box::new('b');
        let boxed_int = Box::new(345);
        let fn_int = noop();
        unsafe {
            println!("{:032b}", *fn_int);
        }

        println!("Global {:p}", &GLOBAL as *const i32);
        println!("local_str  {:p}", local_str as *const str);
        println!("local_int  {:p}", &local_int as *const i32);
        println!("Boxed_int  {:p}", Box::into_raw(boxed_int));
        println!("Boxed_str  {:p}", Box::into_raw(boxed_str));
        println!("fn_int  {:p}", fn_int);

        
    }
//}