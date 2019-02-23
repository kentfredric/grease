use std::{iter::Iterator, option::Option};

pub trait OptFilter: Iterator {
    fn filter_oks<F, T, E>(self, f: F) -> OptFilterIterator<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(&T) -> bool,
    {
        OptFilterIterator { iter: self, filter: f }
    }
    fn map_oks<F, T, TT, E>(self, f: F) -> OptMapIterator<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(&T) -> Result<TT, E>,
    {
        OptMapIterator { iter: self, mapper: f }
    }
}

impl<T: ?Sized> OptFilter for T where T: Iterator {}

pub struct OptFilterIterator<I, F> {
    iter:   I,
    filter: F,
}

impl<I, F, T, E> Iterator for OptFilterIterator<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&T) -> bool,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iterator_return = self.iter.next();
            match &iterator_return {
                None => return iterator_return,
                Some(item_result) => match item_result {
                    Ok(item) => {
                        if (self.filter)(item) {
                            return iterator_return;
                        } else {
                            continue;
                        }
                    },
                    Err(_) => return iterator_return,
                },
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub struct OptMapIterator<I, F> {
    iter:   I,
    mapper: F,
}

impl<I, F, T, TT, E> Iterator for OptMapIterator<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&T) -> Result<TT, E>,
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    type Item = Result<TT, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let iterator_return = self.iter.next();
        if iterator_return.is_none() {
            return None;
        }
        let inner = iterator_return.unwrap();
        if inner.is_err() {
            return Some(Err(inner.unwrap_err()));
        }
        let rval = inner.unwrap();
        return Some((self.mapper)(&rval));
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
