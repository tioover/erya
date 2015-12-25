use std::path::Path;
use std::thread::{JoinHandle, spawn};
use glium::Display;


pub trait Resource: Sized {
    type LoadData: Send + 'static; 

    fn load<P: AsRef<Path>>(path: P) -> Self::LoadData;
    
    fn get<P: AsRef<Path> + Send + 'static>(path: P) -> LoadHandler<Self> {
        LoadHandler(spawn(move || {
            Self::load(path)
        }))
    }
    fn generate(display: &Display, data: Self::LoadData) -> Self;
}


pub struct LoadHandler<T: Resource>(pub JoinHandle<T::LoadData>);


impl<T: Resource> LoadHandler<T> {
    pub fn unwrap(self, display: &Display) -> T {
        let data = self.0.join().unwrap();
        T::generate(display, data)
    }
}