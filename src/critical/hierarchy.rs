pub trait Hierarchy {
    fn decompose(&self) -> Vec<String>;

    fn composition(&self) -> Vec<String>;
}
