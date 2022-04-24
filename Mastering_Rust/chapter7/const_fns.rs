const fn salt(a: u32) -> u32 {
    0xdeadbeef ^ a
}

const fn test() -> u32 {
    23
}

const TESTME: u32 = test();

const CHECKSUM: u32 = salt(23);

fn main() {
    println!("{}", CHECKSUM);
    println!("{}", TESTME);
}