use std::mem;

// This first chapter of Learning Rust with too many linked lists goes like this:
// It introduces some implementations for a single linked list and discusses
// their shortcomings. For the last implementation presented, they
// define the push and pop methods and test them.
//
// In order to decrease the cognitive load
// (https://www.scotthyoung.com/blog/2022/01/04/cognitive-load-theory/)
// and for practice, I implemented these methods for the first proposed
// implementation. The details are almost the same
// but doing this first I (a beginner) could
// focus separately on the method details
// and then on the list details.

// this is our first attemps
// it didn't compile because List is
// a recursive type, which therefore has
// infinite size (actually this means it might have
// infinite size). This is a problem because a struct
// is stored on the stack, and compiler needs to
// know its size (and it has to be fixed)
//    pub enum List {
//        Empty,
//        Elem(i32, List),
//    }

// now the size of List is fixed,
// because Box<List> is a kind of a pointer
//
#[derive(PartialEq, Debug)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

// this implementation is not good because:
// the first element of the list will be stored
// on the stack, [Elem A, ptr], the others will be
// stored on the heap, (Elem B, ptr), (Elem C, ptr),...,
// (Empty, junk)

// but we will implement some methods on it, for the aforementioned reasons
impl List {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn push(&mut self, item: i32) {
        *self = match self {
            List::Empty => List::Elem(item, Box::new(List::Empty)),
            _ => List::Elem(item, Box::new(mem::replace(self, List::Empty))),
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(self, List::Empty) {
            List::Empty => None,
            List::Elem(item, boxedlist) => {
                *self = *boxedlist;
                Some(item)
            }
        }
    }
}
