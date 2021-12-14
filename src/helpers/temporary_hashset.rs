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
    /// Inserts an element into the hashset, returning a guard.
    /// When the guard is destroyed, the element gets removed again.
    ///
    /// This method is specifically tailored to be used in recursive algorithms,
    /// like flood fills or path searches, that need to keep track of already visited items.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the insertion was successful (e.g. the item did not exist yet),
    /// and the Temporary HashSet that includes the given item.
    ///
    /// The Temporary HashSet can be dereferenced to a normal hashset.
    ///
    /// This is a zero-copy operation; the hashset returned by dereferencing the temporary hashset
    /// is the original hashset, not a copy. It just had to go through the temporary hashset due
    /// to mutability ownership.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to be inserted temporarily.
    ///
    fn temporary_insert(&mut self, item: T) -> (bool, TemporaryHashSet<T>);
}

impl<T> HashSetExt<T> for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    fn temporary_insert(&mut self, el: T) -> (bool, TemporaryHashSet<T>) {
        let temp_hashset = TemporaryHashSet::new(self, el);
        (temp_hashset.inserted, temp_hashset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut set = HashSet::from([1, 2, 3]);

        assert_eq!(set, HashSet::from([1, 2, 3]));

        {
            let (insert_succeeded, temporary_set) = set.temporary_insert(4);

            assert!(insert_succeeded);
            assert_eq!(*temporary_set, HashSet::from([1, 2, 3, 4]));
        }

        assert_eq!(set, HashSet::from([1, 2, 3]));

        {
            let (insert_succeeded, temporary_set) = set.temporary_insert(3);

            assert!(!insert_succeeded);
            assert_eq!(*temporary_set, HashSet::from([1, 2, 3]));
        }

        assert_eq!(set, HashSet::from([1, 2, 3]));
    }
}
