use std::convert::AsRef;
use std::path::Path;
use std::rc::Rc;
use std::any::Any;
use std::thread;
use std::cell::RefCell;
use std::collections::{ HashSet, HashMap };
use glium::Display;
use glium::texture::RawImage2d;
use image;
use image::{DynamicImage, GenericImage};
use queue::{ Queue, Event };
use id::Id;
use texture::Texture;


pub struct TextureManager<'display>
{
    display: &'display Display,
    queue: Rc<Queue<Event>>,
    load_jobs: RefCell<HashSet<Id>>,
    textures: RefCell<HashMap<Id, Rc<Texture>>>,
}


impl<'display> TextureManager<'display>
{
    pub fn new(display: &'display Display, queue: Rc<Queue<Event>>)
        -> TextureManager<'display>
    {
        TextureManager
        {
            display: display,
            queue: queue,
            load_jobs: RefCell::new(HashSet::new()),
            textures: RefCell::new(HashMap::new()),
        }
    }

    pub fn load<P>(&self, path: P) -> Id
        where P: AsRef<Path> + Send + 'static
    {
        let id = Id::new();
        let sender = self.queue.sender.clone();
        {
            let mut jobs = self.load_jobs.borrow_mut();
            jobs.insert(id);
        }
        thread::spawn(move || sender.send((id, 
                if let Ok(image) = image::open(path) { Event::Data(encode(&image)) }
                else { Event::Failure }
            )).unwrap());
        return id;
    }

    pub fn get(&self, id: &Id) -> Option<Rc<Texture>>
    {
        self.textures.borrow().get(id).map(|x| x.clone())
    }

    pub fn handle(&self, (id, event): (Id, Event)) -> Option<(Id, Event)>
    {
        if !self.load_jobs.borrow().contains(&id) { Some((id, event)) }
        else 
        {
            match event
            {
                Event::Data(data) =>
                {
                    let data = *data.downcast::<RawImage2d<'static, u8>>().unwrap();
                    let mut textures = self.textures.borrow_mut();
                    textures.insert(id, Rc::new(Texture::with_id(self.display, data, id)));
                }
                _ => panic!(),
            };
            None
        }
    }
}


fn encode(image: &DynamicImage) -> Box<Any+Send>
{
    Box::new(RawImage2d::from_raw_rgba_reversed(
        image.raw_pixels(),
        image.dimensions()))
}


