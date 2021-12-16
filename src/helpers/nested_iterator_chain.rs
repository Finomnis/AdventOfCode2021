//#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NestedIteratorChain<Out: Iterator, In: Iterator, F>
where
    F: FnMut(In::Item) -> Out,
{
    pub(crate) iter: In,
    generating_iter: Option<Out>,
    f: F,
}

impl<Out: Iterator, In: Iterator, F> NestedIteratorChain<Out, In, F>
where
    F: FnMut(In::Item) -> Out,
{
    pub(crate) fn new(iter: In, f: F) -> NestedIteratorChain<Out, In, F> {
        let mut obj = NestedIteratorChain {
            iter,
            generating_iter: None,
            f,
        };

        obj.generating_iter = obj.iter.next().map(|el| (obj.f)(el));

        obj
    }
}

impl<Out: Iterator, In: Iterator, F> Iterator for NestedIteratorChain<Out, In, F>
where
    F: FnMut(In::Item) -> Out,
{
    type Item = Out::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Attempt #1, everything is fine
        if let Some(gen_iter) = &mut self.generating_iter {
            if let Some(next) = gen_iter.next() {
                return Some(next);
            }
        } else {
            return None;
        }

        // Current iter ran dry, refresh
        self.generating_iter = self.iter.next().map(|el| (self.f)(el));

        // Attempt #2, retry after refreshing
        if let Some(gen_iter) = &mut self.generating_iter {
            if let Some(next) = gen_iter.next() {
                return Some(next);
            }
        }

        None
    }
}

pub trait ChainNestedIterator: Iterator {
    fn chain_nested_iterator<Out: Iterator, F>(self, f: F) -> NestedIteratorChain<Out, Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Out,
    {
        NestedIteratorChain::new(self, f)
    }
}

impl<T: ?Sized> ChainNestedIterator for T where T: Iterator {}
