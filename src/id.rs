use time::precise_time_ns;
use rand;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Id {
	time: u64,
	random: u64,
}


impl Id {
    pub fn new() -> Id {
        Id {
        	time: precise_time_ns(),
        	random: rand::random(),
        }
    }
}

