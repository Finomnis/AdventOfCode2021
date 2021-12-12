use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Deref, DerefMut},
};

pub struct TemporaryHashSet<'a, T>
where
    T: Eq + Hash,
{
    hashset: &'a mut HashSet<T>,
    value: T,
    inserted: bool,
}

impl<'a, T> Deref for TemporaryHashSet<'a, T>
where
    T: Eq + Hash,
{
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        self.hashset
    }
}
impl<'a, T> DerefMut for TemporaryHashSet<'a, T>
where
    T: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.hashset
    }
}

impl<'a, T> TemporaryHashSet<'a, T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(hashset: &'a mut HashSet<T>, value: T) -> Self {
        let inserted = hashset.insert(value.clone());

        Self {
            hashset,
            value,
            inserted,
        }
    }
}

impl<'a, T> Drop for TemporaryHashSet<'a, T>
where
    T: Eq + Hash,
{
    fn drop(&mut self) {
        if self.inserted {
            self.hashset.remove(&self.value);
        }
    }
}

pub trait HashSetExt<T>
where
    T: Eq + Hash,
{
    fn temporary_insert<'a>(&'a mut self, el: T) -> (bool, TemporaryHashSet<'a, T>);
}

impl<T> HashSetExt<T> for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    fn temporary_insert<'a>(&'a mut self, el: T) -> (bool, TemporaryHashSet<'a, T>) {
        let temp_hashset = TemporaryHashSet::new(self, el);
        (temp_hashset.inserted, temp_hashset)
    }
}
