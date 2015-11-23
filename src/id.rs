use time;
use rand;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Id {
    time: time::Timespec,
    random: u32,
}


impl Id {
    pub fn new() -> Id {
        Id {
            time: time::now().to_timespec(),
            random: rand::random(),
        }
    }
}

