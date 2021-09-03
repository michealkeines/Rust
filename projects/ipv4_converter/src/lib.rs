fn helper(i: i32,val: u32) -> u32 {
    if val == 48 {
        return 0;
    } else {
        return u32::pow(2,i as u32);
    }
}
fn int32_to_ip(int: u32) -> String {
    
    let a = format!("{:b}",int);
   // println!("{}",a);
    let b = (0..(32-a.len())).map(|_| "0").collect::<String>();
  //  println!("{}",b);
    let c = format!("{}{}",b,a);
 //   println!("{}",c);
  //  println!("{} : {}",c, c.len());
    let k: Vec<char> = c.chars().collect();
    let mut y: i32 = 7;
    let mut result: Vec<u32> = vec![];
    let mut current = 0;
    for j in 0..32 {
        //println!("{}",i as u32);

        if j < 8 {
            current = current + helper(y,(k[j] as u32));
            y = y - 1;
        } else if j < 16 {

            current = current + helper(y,(k[j] as u32));
            y = y - 1;
        } else if j < 24 {
            current = current + helper(y,(k[j] as u32));
            y = y - 1;
        } else if j < 32 {

            current = current + helper(y,(k[j] as u32));
            y = y - 1;
        } else {

        }
        if y < 0 {
            y = 7;
            result.push(current);
            current = 0;
        }
    }
  //  println!("{:?}",result);
    format!("{}.{}.{}.{}",result[0],result[1],result[2],result[3])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(32), "0.0.0.32");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
