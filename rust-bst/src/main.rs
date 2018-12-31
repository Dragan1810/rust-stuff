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

                match node.element {
                    el if el == element => return false,
                    el if el < element => {
                        node.right = Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}));
                        return true
                        },
                    el if el > element => {
                        node.left = Link::More(Box::new(Node {element, left:Link::Empty, right: Link::Empty}));
                        return true
                        },
                    _ => return false
                }
            }
        }
    }

   // fn search(&self, element:i32) -> bool { false }
}



fn main() {
    let mut bst = BST::new();
    bst.insert(5);
    bst.insert(5);
    bst.insert(6);
    bst.insert(3);
    bst.insert(2);
    println!("{:#?}", bst)
}
