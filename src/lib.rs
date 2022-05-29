pub mod zeroth;

#[cfg(test)]
mod tests {
    use crate::zeroth::List;

    #[test]
    fn new_lists() {
        assert_eq!(List::new(), List::Empty);
    }

    #[test]
    fn push_onto_lists() {
        // List of one element
        let mut xs = List::new();
        xs.push(5);
        assert_eq!(xs, List::Elem(5, Box::new(List::Empty)));
        assert_ne!(xs, List::Elem(6, Box::new(List::Empty)));

        // List of more elements
        let mut ys = List::new();
        ys.push(8);
        ys.push(13);
        assert_eq!(
            ys,
            List::Elem(13, Box::new(List::Elem(8, Box::new(List::Empty))))
        );
    }

    #[test]
    fn pop_off_lists() {
        let mut ys = List::new();
        ys.push(8);
        ys.push(13);
        assert_eq!(ys.pop(), Some(13));
        assert_eq!(ys, List::Elem(8, Box::new(List::Empty)));
        ys.pop();
        assert_eq!(ys.pop(), None);
        assert_eq!(ys, List::Empty);
    }
}
