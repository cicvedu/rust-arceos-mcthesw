use core::ops::{Deref,DerefMut};

pub struct NetFilter<T> {
    pub inner: T,
}

impl<T> Deref for NetFilter<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NetFilter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}