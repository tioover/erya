//! Unique identifier.

use time;
use rand;


/// Unique Identifier.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub struct Id
{
    random: u32,
    time: time::Timespec,
}


impl Id
{
    pub fn new() -> Id
    {
        Id
        {
            random: rand::random(),
            time: time::now().to_timespec(),
        }
    }
}

