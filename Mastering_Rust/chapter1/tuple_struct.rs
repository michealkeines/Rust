struct Color(u8, u8, u8);

fn main() {
    let white = Color(255, 255, 255);

    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red value : {}", red);
    println!("Green value : {}", green);
    println!("Blue value : {}", blue);

    let orange = Color(255, 165, 0);

    let Color(r, g, b) = orange;

    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    let Color(r, _, b) = orange;
}
