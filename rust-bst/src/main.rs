use std::cmp::Ordering;

#[derive(Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>
}

impl Eq for Node<i32> {

}

impl Ord for Node<i32> {
    fn cmp(&self, other: &Node<i32>) -> Ordering {
        self.element.cmp(&other.element)
    }
}

impl PartialEq for Node<i32> {
    fn eq(&self, other: &Node<i32>) -> bool {
        self.element == other.element
    }
}

impl PartialOrd for Node<i32> {
    fn partial_cmp(&self, other: &Node<i32>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialEq + PartialOrd> Node<T> {
    fn empty_node(&mut self, element:T) -> Link<T> {
        Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}))
    }

    fn insert(&mut self, element:T) -> Option<Link<T>> {
        match self.element {
                    el if el == element => None,
                    el if el < element => {
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
                    el if el > element => {
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

impl<T> BST<T> {
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
    println!("{:#?}", bst)
}
