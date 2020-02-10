use std::mem;

pub trait Updatable: Default {
    fn self_update<F>(&mut self, func: F) -> ()
        where F: FnOnce(Self) -> Self + Sized
    {
        let self_taken = mem::replace(self, Self::default());
        mem::replace(self, func(self_taken));
    }
}
impl<T> Updatable for T where T: Default {}
