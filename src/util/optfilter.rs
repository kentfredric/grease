use std::{iter::Iterator, option::Option};

pub trait OptFilter: Iterator {
    fn filter_oks<F, T, E>(self, f: F) -> FilterOks<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(&T) -> bool,
    {
        FilterOks { iter: self, filter: f }
    }
    fn map_oks<F, T, TT, E>(self, f: F) -> MapOks<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(&T) -> Result<TT, E>,
    {
        MapOks { iter: self, mapper: f }
    }
    fn extract_errs<F, T, E>(self, f: F) -> ExtractErrs<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(E) -> (),
    {
        ExtractErrs { iter: self, handler: f }
    }
}

impl<T: ?Sized> OptFilter for T where T: Iterator {}

pub struct FilterOks<I, F> {
    iter:   I,
    filter: F,
}

impl<I, F, T, E> Iterator for FilterOks<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&T) -> bool,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iterator_return = self.iter.next();
            match &iterator_return {
                Some(Ok(item)) => {
                    if (self.filter)(item) {
                        return iterator_return;
                    } else {
                        continue;
                    }
                },
                _ => return iterator_return,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub struct MapOks<I, F> {
    iter:   I,
    mapper: F,
}

impl<I, F, T, TT, E> Iterator for MapOks<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&T) -> Result<TT, E>,
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    type Item = Result<TT, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let iterator_return = self.iter.next();
        match iterator_return {
            None => None,
            Some(Err(e)) => Some(Err(e)),
            Some(Ok(i)) => Some((self.mapper)(&i)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub struct ExtractErrs<I, F> {
    iter:    I,
    handler: F,
}
impl<I, F, T, E> Iterator for ExtractErrs<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(E) -> (),
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iterator_return = self.iter.next();
            match iterator_return {
                None => return None,
                Some(Ok(i)) => return Some(i),
                Some(Err(e)) => {
                    (self.handler)(e);
                    continue;
                },
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
