pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    #[allow(dead_code)]
    fn new(outer: O) -> Self {
        Self { outer, inner: None }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(val) = inner_iter.next() {
                    return Some(val);
                }
            }

            let inner = self.outer.next()?.into_iter();
            self.inner = Some(inner);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn iterate_over_empty() {
        assert_eq!(Flatten::new(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn iterate_over_single_inner_element() {
        assert_eq!(Flatten::new(std::iter::once(vec![1])).count(), 1);
    }

    #[test]
    fn iterate_over_two_outer_elements() {
        assert_eq!(
            Flatten::new(vec![vec![1], vec![2]].iter().into_iter()).count(),
            2
        );
    }

    #[test]
    fn iterate_over_two_inner_elements() {
        assert_eq!(Flatten::new(std::iter::once(vec![1, 2])).count(), 2);
    }
}
