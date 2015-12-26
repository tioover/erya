use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{Receiver, channel};
use glium::Display;


pub type Key = PathBuf;


pub trait Resource: Sized {
    type LoadData: Send + 'static; 

    fn load<P: AsRef<Path>>(path: &P) -> Self::LoadData;

    fn get<P: AsRef<Path> + Send + 'static>(path: P) -> LoadHandler<Self> {
        LoadHandler(spawn(move || {
            Self::load(&path)
        }))
    }
    fn generate(display: &Display, data: Self::LoadData) -> Self;
}


pub struct LoadHandler<T: Resource>(pub JoinHandle<T::LoadData>);


impl<T: Resource> LoadHandler<T> {
    pub fn unwrap(self, display: &Display) -> T {
        let LoadHandler(handler) = self;
        T::generate(display, handler.join().unwrap())
    }
}



pub struct Queue<'display, T: Resource> {
    display: &'display Display,
    receiver: Receiver<(T::LoadData, Key)>,
    keys: Vec<Key>,
    pub received: HashMap<Key, Rc<T>>,
}


pub enum RecvState {
    Empty,
    Got,
    NotGot,
}


impl<'a, T: Resource> Queue<'a, T> {
    pub fn new(display: &'a Display, keys: Vec<Key>) -> Queue<'a, T> {
        let (tx, rx) = channel();
        for key in &keys {
            let tx = tx.clone();
            let key = key.clone();
            spawn(move || tx.send((T::load(&key), key)));
        }
        Queue {
            display: display,
            receiver: rx,
            keys: keys,
            received: HashMap::new(),
        }
    }

    pub fn try_recv(&mut self) -> RecvState {
        if self.is_empty() { return RecvState::Empty }
        if let Ok((data, key)) = self.receiver.try_recv() {
            let data = T::generate(self.display, data);
            for i in 0..self.keys.len() {
                if self.keys[i] == key {
                    self.keys.swap_remove(i);
                    break
                }
            }
            self.received.insert(key, Rc::new(data));
            RecvState::Got
        }
        else {
            RecvState::NotGot
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}


