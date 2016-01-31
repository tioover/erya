//! Resource asynchronous loader.

use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::{ JoinHandle, spawn };
use std::sync::mpsc::{ Receiver, channel };
use glium::Display;


pub type Key = PathBuf;


/// Loadable resource types.
pub trait Resource: Sized
{
    /// Middle data.
    type LoadData: Send + 'static;

    /// Synchronize load.
    fn load<P: AsRef<Path>>(path: &P) -> Self::LoadData;

    /// Asynchronous load.
    fn get<P: AsRef<Path> + Send + 'static>(path: P) -> LoadHandler<Self>
    {
        LoadHandler(spawn(move || {Self::load(&path)}))
    }

    /// Build resource object from middle load data.
    fn generate(display: &Display, data: Self::LoadData) -> Self;
}


/// The handler of load thread.
pub struct LoadHandler<T: Resource>(pub JoinHandle<T::LoadData>);


impl<T: Resource> LoadHandler<T>
{
    /// Get data.
    pub fn unwrap(self, display: &Display) -> T
    {
        let LoadHandler(handler) = self;
        T::generate(display, handler.join().unwrap())
    }
}



/// Asynchronous load queue.
pub struct Queue<'display, T: Resource>
{
    display: &'display Display,
    receiver: Receiver<(T::LoadData, Key)>,
    keys: Vec<Key>,
    pub received: HashMap<Key, Rc<T>>,
}


/// State of the load queue.
#[derive(Clone)]
pub enum QueueState
{
    /// Queue is empty, load process is done.
    Empty,
    Received(Key),
    NotReceived,
}


impl<'a, T: Resource> Queue<'a, T>
{
    /// Create a load queue and start asynchronous load.
    pub fn new(display: &'a Display, keys: Vec<Key>) -> Queue<'a, T>
    {
        let (tx, rx) = channel();
        for key in &keys
        {
            let tx = tx.clone();
            let key = key.clone();
            spawn(move || tx.send((T::load(&key), key)));
        }

        Queue
        {
            display: display,
            receiver: rx,
            keys: keys,
            received: HashMap::new(),
        }
    }

    pub fn try_recv(&mut self) -> QueueState
    {
        if self.is_empty() { return QueueState::Empty }

        if let Ok((data, key)) = self.receiver.try_recv()
        {
            let data = T::generate(self.display, data);
            for i in 0..self.keys.len()
            {
                if self.keys[i] == key
                {
                    self.keys.swap_remove(i);
                    break
                }
            }
            self.received.insert(key.clone(), Rc::new(data));
            QueueState::Received(key)
        }
        else
        {
            QueueState::NotReceived
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.keys.is_empty()
    }
}


