use std::collections::HashMap;



fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut uniq = 0;
    let mut tes = HashMap::new();
    let mut val = vec![];
    for i in 0..triplets.len() {
        for j in 0..3 {
            let k = triplets[i][j];
            println!("{}",k);
            if tes.contains_key(&k) {
                continue;
            } else {
                let v: Vec<char> = vec![];
                tes.insert(k, v);
                val.push(k);
            }
        }
    }
    println!("{}", tes.len());
    

    println!("{:?}",val);

    let mut c: Vec<Vec<char>> = Vec::new();
    for i in 0..val.len() {
        let mut tmp: Vec<char> = Vec::new();
        for j in 0..triplets.len() {     
            if triplets[j][0] == val[i] {
                let mut foundv1 = false;
                let mut foundv2 = false;
                for k in 0..tmp.len() {
                    if triplets[j][1] == tmp[k] {
                        foundv1 = true;
                    }
                    if triplets[j][2] == tmp[k] {
                        foundv2 = true;
                    }
                }
                if foundv1 == false {
                    tmp.push(triplets[j][1]);
                }
                if foundv2 == false {
                    tmp.push(triplets[j][2]);
                }
            }
            if triplets[j][1] == val[i] {
                let mut foundv1 = false;
                for k in 0..tmp.len() {
                    if triplets[j][2] == tmp[k] {
                        foundv1 = true;
                    }

                }
                if foundv1 == false {
                    tmp.push(triplets[j][2]);
                }
            }
        }
        c.push(tmp);
    }

    println!("{:?}",c);

    let mut temp = val.clone();
    let mut changed = true;
    println!("asf");
    while changed == true {
        for i in 0..val.len() { // random array with uniq values
            let mut index = 0;
            for l in 0..temp.len() {
                if temp[i] == val[i] {
                    index = l;
                }
            }
            for j in 0..c[i].len() { //check array
                let mut u = 0;
                for l in 0..temp.len() {
                    if temp[l] == c[i][j] {
                        u = l;
                    }
                }
                
                if u < index {
                    helper(index, u, &mut temp);
                    changed = true;
                } else {
                    println!("false");
                    changed = false;
                }
            }
        }
    }
    println!("{:?}",temp);
    "test".to_string()
}

fn helper(i: usize, j: usize, arr: &mut Vec<char> ) {
    while i < j {
        let temp = arr[i];
        arr[i] = arr[i+1];
        arr[i+1] = temp; 
        println!("{:?}",arr);
    }
}





#[test]
fn example_test() {
  assert_eq!(recover_secret(vec![ 
  ['t','u','p'],
  ['w','h','i'],
  ['t','s','u'],
  ['a','t','s'],
  ['h','a','p'],
  ['t','i','s'],
  ['w','h','s']])
  , "whatisup");
}