pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct BinaryTree {
    pub root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(TreeNode {
            value,
            left: None,
            right: None,
        });

        match self.root {
            Some(ref mut node) => {
                let mut current = node;
                loop {
                    if value < current.value {
                        if let Some(ref mut left) = current.left {
                            current = left;
                        } else {
                            current.left = Some(new_node);
                            break;
                        }
                    } else {
                        if let Some(ref mut right) = current.right {
                            current = right;
                        } else {
                            current.right = Some(new_node);
                            break;
                        }
                    }
                }
            }
            None => {
                self.root = Some(new_node);
            }
        }
    }
}

// TODO: Implement other binary tree operations (e.g., search, delete).
// TODO: Write unit tests for the binary tree data structure.