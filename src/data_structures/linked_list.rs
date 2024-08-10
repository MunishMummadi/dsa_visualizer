pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Box::new(ListNode { value, next: None });
        match self.head {
            Some(ref mut node) => {
                let mut current = node;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }
}

// TODO: Implement other linked list operations (e.g., delete, search).
// TODO: Write unit tests for the linked list data structure.