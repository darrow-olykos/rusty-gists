/// Credit to Hayato Ohhashi at https://github.com/o8vm/ods.
/// I'm looking at Hayato's code while I'm reading this book, there is not currently a Rust version of the ODS book
pub trait Queue<T> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> T;
}

pub trait Stack<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}

pub trait PriorityQueue<T> {
    fn add(&mut self, x: T, priority: usize);
    fn delete_min(&mut self) -> T;
}

pub trait Deque<T> {
    fn add_first(&mut self, x: T);
    fn remove_first(&mut self) -> T;
    fn add_last(&mut self, x: T);
    fn remove_last(&mut self) -> T;
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T);
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> T;
}

pub trait USet<T> {
    fn size() -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
}

pub trait SSet<T> {
    fn size() -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
}
