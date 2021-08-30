use std::error::Error;
use std::collections::HashMap;
use std::env;

pub struct input<T> {
    pub place: T,
    pub current_time: u32
}

pub fn get_place<T: ToString>(val: T) -> u32 {
    let mut places = HashMap::new();

    places.insert("london".to_string(),1);
    places.insert("newyork".to_string(),1);
    places.insert("1".to_string(),1);
    places.insert("2".to_string(),2);

    match places.get(&val.to_string()) {
        Some(place) => *place,
        None => 0
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Enter the name or number of the location: ");
    let args: Vec<String> = env::args().collect();
    
    Ok(())

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input() {
        let a = 2;
        let b = String::from("london");
        let c = String::from("newyork");
        assert_eq!(2,get_place(a));
        assert_eq!(1,get_place(b));
        assert_eq!(2,get_place(c));

    }
}