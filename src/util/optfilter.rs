use std::{iter::Iterator, option::Option};

pub trait OptFilter: Iterator {
    fn filter_oks<F, T, E>(self, f: F) -> OptFilterIterator<Self, F>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        F: FnMut(&T) -> bool,
    {
        OptFilterIterator { iter: self, filter: f }
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
