use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

struct Cacher<T>  where T: Fn(HashMap<u32,u32>) -> HashMap<u32,u32> {
    calculation: T,
    value: HashMap<u32,u32>,
}

impl<T> Cacher<T> 
     where T: Fn(HashMap<u32,u32>) -> HashMap<u32,u32>
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, 
            value: HashMap::new(), 
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        let result = self.value.get(&arg);
       match result {
           Some(v) => *v,
           None => {
            self.value.insert(arg,arg);
            arg
           },
       }
    }
}
 

fn generate_workout(intensity: u32, random_number: u32) {
    /* 
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    */
    let mut expensive_closure = Cacher::new(|map| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        map
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut result = Cacher::new(|a|a);

        let v1 = result.value(1);
        let v2 = result.value(2);
        assert_eq!(v2,2);

    }
}
