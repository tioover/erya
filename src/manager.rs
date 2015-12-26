use std::collections::BTreeMap;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::rc::{Rc, Weak};
use std::thread::spawn;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use glium::Display;
use loader::Resource;


pub type Map<T> = BTreeMap<PathBuf, Arc<T>>;


pub struct Manager<'display, T: Resource + Send + 'static> {
    pub map: Arc<Mutex<Map<T>>>,
    root_path: PathBuf,
    display: &'display Display,
    sender: Sender<T::LoadData>,
    receiver: Receiver<T::LoadData>,
}




impl<'a, T: Resource + Send + Sync + 'static> Manager<'a, T> {
    pub fn new<P: AsRef<Path>>(display: &'a Display, path: P) -> Manager<'a, T> {
        let (tx, rx) = channel();
        Manager {
            map: Arc::new(Mutex::new(Map::new())),
            root_path: path.as_ref().to_path_buf(),
            display: display,
            receiver: rx,
            sender: tx,
        }
    }



    pub fn load<P: AsRef<Path>>(&self, path: P) {
        let key = self.root_path.join(path);
        let tx = self.sender.clone();
        let map = self.map.clone();
        spawn(move || {
            let data = T::load(key);
            let map = map.lock().unwrap();
        });
    }
}


