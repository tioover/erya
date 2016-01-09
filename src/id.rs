use time;
use rand;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Id {
    random: u32,
    time: time::Timespec,
}


impl Id {
    pub fn new() -> Id {
        Id {
            random: rand::random(),
            time: time::now().to_timespec(),
        }
    }
}

