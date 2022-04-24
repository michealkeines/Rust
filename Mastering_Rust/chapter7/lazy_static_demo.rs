#[maro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref ITEMS: Mutex<Vec<u64>> = {
        let mut v = vec![];
        v.push(9);
        v.push(2);
        v.push(1);
        Mutex::new(v)
    }
}

fn main() {

}