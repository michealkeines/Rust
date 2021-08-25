use std::thread;
use std::time::Duration;
use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

//https://users.rust-lang.org/t/rust-book-chapter-13-difficulty-understanding-how-to-improve-the-cacher-type/30856/5
struct Cacher<T, K, V> 
    where T: Fn(&K) -> V,
    K: std::cmp::Eq + std::hash::Hash
{
    calculation: T,
    value: HashMap<K, V>
}

impl<T, K, V> Cacher<T, K, V>
    where T: Fn(&K) -> V,
          K: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
          V: std::fmt::Debug
{
    pub fn new(calculation: T) -> Cacher<T, K, V>
    {
        let t = HashMap::new();
        Cacher {
            calculation,
            value: t
        }
    }

    pub fn value(&mut self, arg: K) -> &V {
        match self.value.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
	            entry.insert(v)
            }
        }
    }
}

pub fn run(intensity: u32, random_number: u32) -> Result<(),Box<dyn Error>> {
    generate_workout(intensity, random_number);
    Ok(())
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(&intensity));
        }
    }
}


pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    print!("calculating slowly...\n");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        let _a: u32 = 12;
        assert_eq!(_a,simulated_expensive_calculation(12));
    }

    #[test]
    fn cache_test() {
        let mut cac = Cacher::new(|val| *val); // takes in any int value as key and returns its value
        let _a = 1;
        
        let v1 = cac.value(_a);

        assert_eq!(*v1,1);

        let mut cac1 = Cacher::new(|val : &String| {val.len()}); // closure that takes in a string and returns size
        let u = String::from("test");
        let v2 = cac1.value(u);

        assert_eq!(*v2,4);
    }

    #[test]
    fn test_workout_generator() {
        let a: u32 = 10;
        let b: u32 = 7;
        assert_eq!((), generate_workout(a,b));
    }

}
