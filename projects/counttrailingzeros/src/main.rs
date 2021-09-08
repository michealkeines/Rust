fn zeros(n: u64) -> u64 {
    if n < 5 {
        return 0;
    }
    let mut count = 0;
    let mut t = n;
    println!("before = t: {}, count: {}",t,count);
    while t >= 5 {
        t = t / 5;
        count = count + t;
        println!("after = t: {}, count: {}",t,count);
    }
    count
}

fn main() {
        assert_eq!(zeros(0), 0);
       assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
}
