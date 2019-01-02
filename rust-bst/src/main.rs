#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Node {
    element: i32,
    left: Link,
    right: Link
}

impl Node {
    fn empty_node(&mut self, element:i32) -> Link {
        Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}))
    }

    fn insert(&mut self, element:i32) -> Option<Link> {
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
struct BST {
    root: Link
}

impl BST {
    fn new() -> Self {
        BST { root: Link::Empty }
    }

    fn insert(&mut self, element:i32) -> bool {
      match self.root {
            Link::Empty => {
               self.root = Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}));
                true
                },
            Link::More(ref mut node) => {
                    node.insert(element);
                    return true
                }
            }
        }
    }

   // fn search(&self, element:i32) -> bool { false }

fn main() {
    let mut bst = BST::new();
    bst.insert(5);
    bst.insert(6);
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);
    bst.insert(2);
    println!("{:#?}", bst)
}
