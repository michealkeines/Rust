
fn main() {
    let intial_lines = ["On the "," day of Christmas","my true love sent to me","a partridge in a pear tree.","and "];
    let gifts = ["","Two turtle doves,","Three French hens,","Four calling birds,","Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];
    let days = ["","first","second","third","fourth","fiveth","sixth","seventh","eighth","ninth","tenth","eleventh","twelveth"];

    for count in 1..13 {
        println!("");
        if count == 1 {
            println!("{}{}{}",intial_lines[0],days[count],intial_lines[1]);
        } else {
            println!("{}{}{},",intial_lines[0],days[count],intial_lines[1]);
        }
        println!("{}",intial_lines[2]);
        for gift in (1..count).rev() {
            println!("{}",gifts[gift]);
        }
        if count != 1{
            println!("{}{}",intial_lines[4],intial_lines[3]);
        }else{
            println!("{}",intial_lines[3]);
        }

    }
}
