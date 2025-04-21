pub trait IterableEnum {
    type Iter: Iterator<Item = Self>;

    fn iter() -> Self::Iter;
}
