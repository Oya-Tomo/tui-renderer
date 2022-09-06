use std::{time::{Duration, Instant}, thread::sleep};

pub struct Timer {
    rate: Duration,
    last: Instant,    
}

impl Timer {
    pub fn new(rate: Duration) -> Self {
        Timer { rate: rate, last: Instant::now() }
    }

    // pub fn frame_wait(&mut self) -> bool {
    //     let end = self.last.elapsed();
    //     self.last = Instant::now();

    //     if let Some(dur) = self.rate.checked_sub(end) {
    //         sleep(dur);
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }

    pub fn frame_wait(&mut self) -> bool {
        let end = self.last.elapsed();
        self.last = Instant::now();

        let dur = self.rate.checked_sub(end);
        if dur.is_some() {
            sleep(dur.unwrap());
            return true;
        } else {
            return false;
        }
    }
}