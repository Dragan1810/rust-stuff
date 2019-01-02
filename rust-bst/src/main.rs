use std::cmp::PartialOrd;

#[derive(Debug, PartialEq, PartialOrd)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

//type Link = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, PartialOrd)]
struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>
}
/*
type NodeI<T> = Option<Vec<Vec<String>>>;

struct IntoIter<T>(NodeI<T>);

impl NodeI<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    } 
}

*/
impl<T: PartialOrd> Node<T> {
    fn empty_node(&mut self, element:T) -> Link<T> {
        Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}))
    }

    fn insert(&mut self, element:T) -> Option<Link<T>> {
        match self.element {
                    ref el if el < &element => {
                        match self.right {
                            Link::Empty => {
                                self.right = self.empty_node(element);
                                None
                            },
                            Link::More(ref mut node) => {
                                self.right = node.insert(element)?;
                                None
                            }
                        }
                    },
                    ref el if el > &element => {
                        match self.left {
                            Link::Empty => {
                                self.left = self.empty_node(element);
                                None
                            },
                            Link::More(ref mut node) => {
                                   self.left = node.insert(element)?;
                                   None
                            }
                        }
                    },
                    _ => return None
        }
    }
}

#[derive(Debug)]
struct BST<T> {
    root: Link<T>
}

impl<T: PartialOrd> BST<T> {
    fn new() -> BST<T> {
        BST { root: Link::Empty }
    }

    fn insert(&mut self, element:T) -> bool {
      match self.root {
            Link::Empty => {
               self.root = Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}));
                true
                },
            Link::More(ref mut node) => {
                    node.insert(element);
                    true
                }
            }
        }
    }

   // fn search(&self, element:i32) -> bool { false }

fn main() {
    let mut bst = BST::new();
    bst.insert(5);
    bst.insert(1);
    bst.insert(6);
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);
    bst.insert(2);
    println!("{:#?}", bst);

    //let mut iter = bst.into_iter();
    //println!("{:#?}", iter.next())
    
}
