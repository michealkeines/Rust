fn main() {
    let level_0_str = String::from("foo");
    {
        let level_1_number = 9;
        {
            let mut level_2_vector = vec![1, 2, 3];
            level_2_vector.push(level_1_number); // can acccess
        } // level 2 goes out of scope
        level_2_vector.push(4); // no longer exists
    } // level 1 goes out of scope
} // level 0 goes out of scope