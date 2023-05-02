// Good for stack like structures

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<ListNode>>,
}

#[derive(Debug)]
struct ListNode {
    val: i32,
    // option is to avoid infinite loop during creation
    next: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn empty() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, val: i32) {
        let old_head = self.head.take();
        let new_head = Box::new(ListNode {
            val: val,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let old_head = self.head.take();
        if let Some(data) = old_head {
            self.head = data.next;
            return Some(data.val);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        println!("linked list");
        let mut ll = LinkedList::empty();
        ll.push(10);
        ll.push(20);
        ll.push(30);
        dbg!(&ll);
        let x = ll.pop().unwrap();
        dbg!(x);
        dbg!(&ll);
    }
}
