use std::cmp::Ordering;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Arbitrary<T>(pub T);

impl<T> PartialEq for Arbitrary<T> {
    #[inline(always)]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Arbitrary<T> {}

impl<T> PartialOrd for Arbitrary<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Arbitrary<T> {
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}
