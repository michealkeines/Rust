
/// How Floating-Point actually works?

fn main() {
    let a: f32 = -42.42;
    let bits: u32 = a.to_bits();
    println!("{:032b}", bits);
    let sign_bit: u32 = bits >> 31; // this places the 32th bit which is our sign bit in the
    // 11000010001010011010111000010100 -> 00000000000000000000000000000001
    println!("{} , {:032b}", sign_bit, sign_bit);

    // this removes all other value except sign bit and exponent
    // 1 signbit + 8 bits exponent => 32 - 9 = 23
    let exponent_ = bits >> 23; 
    println!("{:032b}", exponent_);
    let exponent_ = exponent_ & 0b11111111; // this removes the sign bit
    println!("{:032b}", exponent_);
    let exponent = (exponent_ as u32) - 127; // here 127 is the standard bias
    println!("{:032b}", exponent); // 00000000000000000000000000000101 -> 5

    println!("{}", (-1.0_f32).powf(0.0));
    println!("{}", -1.0_f32.powf(0.0));

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        println!("\n\n");
        let mask = 1 << i;
        println!("mask: {:032b}", mask);
        let one_at_bit = bits & mask;
        println!("bits: {:032b}", bits);
        println!("shitfed: {:032b}", one_at_bit);

        if one_at_bit != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_- 23.0);
            mantissa += weight;
            println!("i: {}, sub: {}, wight: {}, mantissa: {}",i_, i_ - 23.0,weight,mantissa);
        }
        println!("\n\n");
    }
}