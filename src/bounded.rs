use std::ops::RangeBounds;

pub trait Bounded<T, R>: Default + Sized
where
    R: RangeBounds<T>,
{
    const RANGE: R;

    fn new_or_default(value: T) -> Self;

    fn get(&self) -> &T;
}
