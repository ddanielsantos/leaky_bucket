use std::time::Instant;

struct LeakyBucket {
    capacity: u64,
    leak_rate: u64,
    current_level: u64,
    last_leak_time: Instant,
}

impl LeakyBucket {
    fn new(capacity: u64, leak_rate: u64) -> Self {
        Self {
            capacity,
            leak_rate,
            current_level: 0,
            last_leak_time: Instant::now(),
        }
    }

    fn leak(&mut self) -> () {
        let leaked = self.last_leak_time.elapsed().as_secs() * self.leak_rate;

        if leaked > 0 {
            self.current_level = self.current_level.saturating_sub(leaked);
            self.last_leak_time = Instant::now();
        }
    }

    pub fn add_event(&mut self) -> bool {
        self.leak();

        if self.current_level < self.capacity {
            self.current_level += 1;
            return true;
        }

        false
    }

    pub fn remaining_capacity(&mut self) -> u64 {
        self.leak();
        self.capacity.saturating_sub(self.current_level)
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use std::{thread, time};

    use super::*;

    #[test]
    fn it_works() {
        let mut lbucket = LeakyBucket::new(3, 1);
        let mut rng = rand::thread_rng();

        for x in 0..7 {
            let a = lbucket.add_event();

            if !a {
                println!("dropped {} remain {}", x, lbucket.remaining_capacity());
            } else {
                println!("ran {} remain {}", x, lbucket.remaining_capacity());
            }

            thread::sleep(time::Duration::from_millis(rng.gen_range(100..1000)));
        }

        // thread::sleep(time::Duration::from_millis(1000));

        // let a = lbucket.add_event();

        // if !a {
        //     println!("dropped remain {}", lbucket.remaining_capacity());
        // } else {
        //     println!("ran remain {}", lbucket.remaining_capacity());
        // }
    }
}
