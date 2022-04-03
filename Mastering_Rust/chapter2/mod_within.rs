mod food {
    pub struct Cake;
    struct Smoothie;
    struct Pizza;
}

use food::Cake;

fn main() {
    let eatable = food::Cake;
}