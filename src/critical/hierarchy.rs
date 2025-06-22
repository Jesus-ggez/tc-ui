pub trait Hierarchy {
    fn decompose(&self) -> Vec<String>;

    fn compose(&self) -> Vec<String>;
}
