pub trait Loader<T> {
    fn init() -> Self;
    fn save(&mut self, name : &String, data : T);
    fn load(&self, name : &String) -> Option<&T>;
    fn len(&self) -> usize;
}