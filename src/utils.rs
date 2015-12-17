use std::ops::Deref;


pub enum Ref<'a, B> where B: 'a {
    Borrowed(&'a B),
    Owned(B),
}

impl<'a, B> Deref for Ref<'a, B> where B: 'a {
    type Target = B;
    fn deref(&self) -> &B {
        match self {
            &Ref::Borrowed(x) => x,
            &Ref::Owned(ref x) => x,
        }
    }
}

