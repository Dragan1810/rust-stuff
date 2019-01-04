use std::cmp::PartialOrd;

/*
enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}
*/

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>
}

struct Iter<'a,T: 'a> {
    next: Option<&'a Node<T>>,
}



#[derive(Debug)]
struct IntoIter<T>(BST<T>);

impl<T> BST<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter<'a>(&'a self) -> Iter<'a, T> {
        // |node| &**node ???
        Iter {
            next: self.root.as_ref().map(|node| &**node)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(| node | {
            &node.element

        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|node| node.element) 
    } 
}


impl<T: PartialOrd> Node<T> {
    fn empty_node(&mut self, element:T) -> Link<T> {
        Some(Box::new(Node {element, left:None, right: None}))
    }

    fn insert(&mut self, element:T) -> Option<Link<T>> {
        match self.element {
                    ref el if el < &element => {
                        match self.right {
                            None => {
                                self.right = self.empty_node(element);
                                None
                            },
                            Some(ref mut node) => {
                                self.right = node.insert(element)?;
                                None
                            }
                        }
                    },
                    ref el if el > &element => {
                        match self.left {
                            None => {
                                self.left = self.empty_node(element);
                                None
                            },
                            Some(ref mut node) => {
                                   self.left = node.insert(element)?;
                                   None
                            }
                        }
                    },
                    _ => None
        }
    }
}

#[derive(Debug, Clone)]
struct BST<T> {
    root: Link<T>
}

impl<T: PartialOrd> BST<T> {
    fn new() -> BST<T> {
        BST { root: None }
    }

    fn insert(&mut self, element:T) -> bool {
      match self.root {
            Some(ref mut node) => {
                    node.insert(element);
                    true
                },
            None => {
                self.root = Some(Box::new(Node {element, left:None, right:None}));
                true
                },
            }
        }

    //fn search(&mut self, _element: &T) -> bool { 
    //    true
    // }

    }

    

fn main() {
    let mut bst = BST::new();
    bst.insert(5);
    bst.insert(1);
    bst.insert(6);
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);
    bst.insert(2);
    //println!("{:#?}", bst);

    //bst.search(3);
    for e in bst.iter() {
        println!("{:?}",e)
    }
    
}
