#[derive(Debug)]
struct Times {
    val: Vec<u32>
}

struct TimesIter<'a> {
    index: usize,
    values: &'a Vec<u32>
}

impl<'a> Iterator for TimesIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut temp = None;
        if &self.index < &self.values.len() {
            temp = Some(self.values[self.index]);
            self.index += 1;
        }
        temp
    }
}

impl Times {
    fn new(val: u32) -> Times {
        let mut temp = vec![];
        for i in 0..val as usize {
            temp.push(i as u32);
        }
        Times {
            val: temp
        }
    }

    fn iter(&self) -> TimesIter {
        TimesIter {
            index: 0,
            values: &self.val
        }
    } 
}

fn main() {
    let time = Times::new(10);
    println!("{:?}",time);

    //let mut it = time.iter();

  //  while let Some(i) = it.next() {
      for i in time.iter() {
        print!("{:?}", i);
    }
}