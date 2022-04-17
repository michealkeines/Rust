fn main() {
    unsafe {
        let a = 12;
        let mut ptr = a as *mut u32;

        println!("raw pointer {:?}", ptr);
        println!("value in raw pointer {}", ptr as u32);

        let b = 14;
        ptr = b as *mut u32;

        println!("value in raw pointer {}", ptr as u32);
    }

}