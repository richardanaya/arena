use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct ArenaValueWrapper<T> where T:ArenaAllocator{
    pub index:generational_arena::Index,
    pub phantom: std::marker::PhantomData<T>,
}

impl<T> ArenaValueWrapper<T> where T:ArenaAllocator+'static {
    pub fn load(&self) -> Arc<Mutex<T>> {
        T::load(self.index).clone()
    }
}

impl<T> Drop for ArenaValueWrapper<T> where T:ArenaAllocator {
    fn drop(&mut self) {
        T::deallocate(self.index);
    }
}

pub trait ArenaAllocator {
    fn allocate() -> ArenaValueWrapper<Self> where Self:Sized;
    fn deallocate(index:generational_arena::Index);
    fn load(index:Index) -> Arc<Mutex<Self>>;
}

pub use lazy_static::lazy_static;
pub use generational_arena::{Index,Arena};