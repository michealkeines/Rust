use std::thread;
use std::time::Duration;
use std::error::Error;
use std::collections::HashMap;

struct Cacher<T> 
    where T: Fn(u32) -> u32 
{
    calculation: T,
    value: HashMap<u32,u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        let t: HashMap<u32, u32> = HashMap::new();
        Cacher {
            calculation,
            value: t
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        if self.value.len() == 0 {
            let val: u32 = (self.calculation)(arg);
            self.value.insert(val, val);
            val
        } else {
            match self.value.get(&arg) {
                Some(val) => *val,
                None => {
                    let val = (self.calculation)(arg);
                    self.value.insert(val, val);
                    val
                }
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
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
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
        let a: u32 = 12;
        assert_eq!(12,simulated_expensive_calculation(12));
    }

    #[test]
    fn cache_test() {
        let mut cac = Cacher::new(|val| val);
        let v1 = cac.value(5);
        let v2 = cac.value(1);
        let v3 = cac.value(4);
        let v4 = cac.value(5);

        assert_eq!(v4, 5);
    }

    #[test]
    fn test_workout_generator() {
        let a: u32 = 10;
        let b: u32 = 7;
        assert_eq!((), generate_workout(a,b));
    }

}
