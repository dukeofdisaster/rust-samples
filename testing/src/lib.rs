//! 
//! A simple singly-linked list for rust coobook.
//! 
//! Recipes covered herein:
//! - Documenting code
//! - Testing documentation
//! - Writing tests
//! 

// have to add std to use Rc
use std::rc::Rc;
use std::cell::RefCell;


#[cfg(test)]
mod tests {

    // so we have access to the types defined outside of this module
    // in the same file
    use super::*;
    //extern crate test; // <- this is being deprecated by rust, so #[bench] should also
    // be omitted

    #[test]
    fn test_list_new_empty() { 
      let mut list: List<i32> = List::new_empty();
      assert_eq!(list.length, 0);
      assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_append() { 
      let mut list = List::new_empty();
      list.append(1);
      list.append(1);
      list.append(1);
      list.append(1);
      list.append(1);
      assert_eq!(list.length, 5);
    }

    #[test]
    fn test_list_pop() {
      let mut list = List::new_empty();
      list.append(1);
      list.append(1);
      list.append(1);
      list.append(1);
      list.append(1);
      assert_eq!(list.length, 5);
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.length, 0);
      assert_eq!(list.pop(), None);
    }
}

// node for implementing a singly-linked list.
// derives clone so it's clonable. It's a generic type
// so we can have nodes of multiple types
#[derive(Clone)]
struct Node<T> where T: Sized + Clone {
  value: T,
  next: Link<T>,
}

// link type for making writing easier
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

// an implementation block to define functions
// that Node implements
impl<T> Node<T> where T: Sized + Clone {
  // this func is private by default since 'pub' is not explictly specified
  // returns: a Refcell of struct Node type T
  fn new(value: T) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node {
      value: value,
      next: None,
    }))
  }
}

///
/// A singly-linked list with nodes allocated on the heap using
/// `Rc`s and `RefCell`s.  Below is an image illustrating linked lists
/// 
/// ![](https://upload.wikimedia.org/wikipedia/commons/6/6d/Singly-linked-list.svg)
/// 
/// # Usage
/// 
/// ```
/// let list = List::new_empty();
/// ```
/// 
/// # Panics
/// Never panics
/// 
/// # Safety
/// No unsafe code was used
/// 
/// # Example
/// 
/// ```
/// use testing::List;
/// let mut list = List::new_empty();
/// list.append(10);
/// ```
/// 
// a list with functions to add and remove nodes
#[derive(Clone)]
pub struct List<T> where T: Sized + Clone {
  head: Link<T>,
  tail: Link<T>,
  pub length: usize,
}

// list's imple block
impl<T> List<T> where T: Sized + Clone {
  pub fn new_empty() -> List<T> {
    List { head: None, tail: None, length: 0}
  }

  pub fn append(&mut self, value: T) {
    let new = Node::new(value); 
    match self.tail.take() { 
      Some(old) => old.borrow_mut().next = Some(new.clone()),
      None => self.head = Some(new.clone())
    };
    self.length += 1;
    self.tail = Some(new);
  }

  // &mut self means it's called on the list itself
  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|head| { 
      if let Some(next) = head.borrow_mut().next.take() {
        self.head = Some(next);
      } else {
        self.tail.take();
      } 
      self.length -= 1;
      Rc::try_unwrap(head)
        .ok()
        .expect("Something went wrong")
        .into_inner()
        .value
     })
  }
}
