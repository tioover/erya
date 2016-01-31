//! Program timer and FPS counter.

use std::time::Duration;
use std::collections::VecDeque;
use time::precise_time_ns;



const SECOND: u64 = 1_000_000_000;


/// Program timer and FPS counter.
pub struct Timer
{
    limit: u8,
    frames: VecDeque<u64>,
}


impl Timer
{
    pub fn new() -> Timer
    {
        Timer
        {
            limit: 60,
            frames: VecDeque::with_capacity(128),
        }
    }

    /// Set FPS limit.
    pub fn limit(self, fps: u8) -> Timer
    {
        Timer { limit: fps, ..self }
    }

    fn limit_frame_rate(&self)
    {
        use std::thread::sleep;

        let frame_ns = SECOND / self.limit as u64;
        let delta = self.delta_ns();
        if frame_ns > delta
        {
            sleep(Duration::new(0, (frame_ns - delta) as u32));
        }
    }

    pub fn update(&mut self)
    {
        self.limit_frame_rate();
        let now = precise_time_ns();
        let a_second_ago = now - SECOND;
        while self.frames.front().map_or(false, |t| *t < a_second_ago)
        {
            self.frames.pop_front();
        }
        self.frames.push_back(now);
    }

    pub fn fps(&self) -> usize
    {
        self.frames.len()
    }

    fn delta_ns(&self) -> u64
    {
        self.frames.back().map_or(0, |t| precise_time_ns() - *t)
    }

    pub fn delta(&self) -> Duration { Duration::new(0, self.delta_ns() as u32) }
}


