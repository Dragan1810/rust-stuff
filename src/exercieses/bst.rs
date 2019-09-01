/*
use std::cmp::PartialOrd;


enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>,
}

struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

#[derive(Debug)]
pub struct IntoIter<T>(BST<T>);

impl<T> BST<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // |node| &**node ???
        Iter {
            next: self.root.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| &node.element)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|node| node.element)
    }
}

impl<T: PartialOrd> Node<T> {
    pub fn empty_node(&mut self, element: T) -> Link<T> {
        Some(Box::new(Node {
            element,
            left: None,
            right: None,
        }))
    }

    pub fn insert(&mut self, element: T) -> Option<Link<T>> {
        match self.element {
            ref el if el < &element => match self.right {
                None => {
                    self.right = self.empty_node(element);
                    None
                }
                Some(ref mut node) => {
                    self.right = node.insert(element)?;
                    None
                }
            },
            ref el if el > &element => match self.left {
                None => {
                    self.left = self.empty_node(element);
                    None
                }
                Some(ref mut node) => {
                    self.left = node.insert(element)?;
                    None
                }
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BST<T> {
    root: Link<T>,
}

impl<T: PartialOrd> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, element: T) -> bool {
        match self.root {
            Some(ref mut node) => {
                node.insert(element);
                true
            }
            None => {
                self.root = Some(Box::new(Node {
                    element,
                    left: None,
                    right: None,
                }));
                true
            }
        }
    }

    //fn search(&mut self, _element: &T) -> bool {
    //    true
    // }
}

*/
