//! Asynchronous event queue.

use std::any::Any;
use std::sync::mpsc::{ Receiver, Sender, channel };
use id::Id;

pub struct Queue<E: EventType>
{
    pub receiver: Receiver<(Id, E)>,
    pub sender: Sender<(Id, E)>,
}


impl<E: EventType> Queue<E>
{
    pub fn new() -> Queue<E>
    {
        let (tx, rx) = channel();
        Queue { receiver: rx, sender: tx }
    }
}


pub trait EventType {}


pub enum Event
{
    Data(Box<Any+Send>),
    Failure,
    Update(Id),
}


impl EventType for Event {}


pub enum TimeEvent
{
    /// Current frame should be ended.
    FrameEnd,
    Pulse,
    /// Timing trigger event.
    Trigger,
}

impl EventType for TimeEvent {}
