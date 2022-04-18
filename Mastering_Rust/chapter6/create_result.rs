fn main() {
    
    const my_err: Result<i32, &str> = Err("oh no!");
    let my_result: Result<i32, ()> = Ok(54);
}