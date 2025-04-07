struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }
    
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    fn insert(&mut self, value: i32) {
        self.root = Some(Self::insert_node(self.root.take(), value));
    }
    
    fn insert_node(node: Option<Box<Node>>, value: i32) -> Box<Node> {
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Some(Self::insert_node(n.left.take(), value));
                } else {
                    n.right = Some(Self::insert_node(n.right.take(), value));
                }
                n
            }
            None => Box::new(Node {
                value,
                left: None,
                right: None,
            }),
        }
    }
    
    fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }
    
    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(n) => {
                if n.value == value {
                    true
                } else if value < n.value {
                    Self::search_node(&n.left, value)
                } else {
                    Self::search_node(&n.right, value)
                }
            }
            None => false,
        }
    }
}

// Testes
#[cfg(test)]
mod bst_tests {
    use crate::BST;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();
        
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        assert!(!bst.search(20));
        
        assert!(!bst.is_empty());
    }
}