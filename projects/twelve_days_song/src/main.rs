fn main() {
    println!("The Twelve Days of Christmas");
    println!("-------------------------------");
    println!("");
    let day: [&str;12] = ["first", "second", "third", "fourth", "fith", "sixth", "seeventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts: [&str;12] = ["Twelve fiddlers fiddling","Eleven ladies dancing","Ten pipers piping","Nine drummers drumming","Eight maids a-milking","Seven swans a-swimming","Six geese a-laying","Five gold rings","Four colly birds","Three French hens","Two turtle doves","A partridge in a pear tree"];
    let mut i: usize = 0;
    for d in day.iter() {
        println!("The {} day of Christmas,", d);
        println!("My true love sent to me");
        let mut temp: usize = 11 - i;
        if temp == 11 {
            println!("{}.",gifts[temp]);
        } else{
            while temp <= 11 {
                if temp == 10 {
                    println!("{}, and",gifts[temp]);
                } else {
                    if temp == 11 {
                        println!("{}.",gifts[temp]);
                    } else {
                        println!("{},",gifts[temp]);
                    }
                }
                temp = temp + 1;
            }
        }
        i = i + 1;
        println!("");
    }
}
