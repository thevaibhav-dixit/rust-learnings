pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    #[allow(dead_code)]
    fn new(outer: O) -> Self {
        Self {
            outer,
            front_iter: None,
            back_iter: None,
        }
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
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(val) = inner_iter.next() {
                    return Some(val);
                }
                self.front_iter = None
            }

            if let Some(next_iter) = self.outer.next() {
                self.front_iter = Some(next_iter.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

// pub trait DoubleEndedIterator: Iterator {
// Required method
// fn next_back(&mut self) -> Option<Self::Item>;

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(val) = back_iter.next_back() {
                    return Some(val);
                }
                self.back_iter = None;
            }

            if let Some(back_iter) = self.outer.next_back() {
                self.back_iter = Some(back_iter.into_iter())
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
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
        assert_eq!(Flatten::new(vec![vec![1], vec![2]].into_iter()).count(), 2);
    }

    #[test]
    fn iterate_over_two_inner_elements() {
        assert_eq!(Flatten::new(std::iter::once(vec![1, 2])).count(), 2);
    }

    #[test]
    fn iterate_over_two_inner_elements_from_back() {
        assert_eq!(
            Flatten::new(std::iter::once(vec![1, 2]))
                .rev()
                .collect::<Vec<_>>(),
            vec![2, 1]
        );
    }

    #[test]
    fn iterate_over_two_outer_elements_from_back() {
        assert_eq!(
            Flatten::new(vec![vec![1], vec![2]].into_iter())
                .rev()
                .collect::<Vec<_>>(),
            vec![2, 1]
        );
    }

    #[test]
    fn handles_middle_element_edge_case() {
        let mut v = Flatten::new(vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter());

        assert_eq!(v.next(), Some(1));
        assert_eq!(v.next_back(), Some(6));
    }
}
