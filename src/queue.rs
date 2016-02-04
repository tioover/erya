//! Asynchronous event queue.

use std::any::Any;
use std::sync::mpsc::{ Receiver, Sender, channel };
use id::Id;


pub struct Queue<T>
{
    pub receiver: Receiver<(Id, T)>,
    pub sender: Sender<(Id, T)>,
}


impl<T> Queue<T>
{
    pub fn new() -> Queue<T>
    {
        let (tx, rx) = channel();
        Queue { receiver: rx, sender: tx }
    }
}


/// General event.
pub enum Event
{
    Data(Box<Any+Send>),
    /// Task failure.
    Failure,
    Update(Id),
}


/// Timing event.
pub enum TimeEvent
{
    /// Current frame should be ended.
    FrameEnd,
    Pulse,
    /// Timing trigger event.
    Trigger,
}


pub trait EventHandler
{
    /// Hendle asynchronous event, if match return **None** and consume event.
    fn pipe(&self, (Id, Event)) -> Option<(Id, Event)>;
}

