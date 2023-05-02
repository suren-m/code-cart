// exactly one root
// no more than 2 child nodes
// exactly one way to get to a node
// for BST, left node is always smaller

#[derive(Debug)]
struct Tree {
    root: Option<Box<TreeNode>>,
}

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl Tree {
    fn empty() -> Self {
        Tree { root: None }
    }
    fn new(val: i32) -> Self {
        Tree {
            root: Some(Box::new(TreeNode {
                val: val,
                left: None,
                right: None,
            })),
        }
    }
    fn dfs(&self) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<&Box<TreeNode>> = Vec::new();
        // traverse all the way to depth. if no children, then jump across
        // add to res when item is popped from stack

        if let Some(root_val) = &self.root {
            stack.push(&root_val);
        } else {
            return res;
        }

        while !stack.is_empty() {
            //dbg!(&res);
            //dbg!(&stack);
            let curr = stack.pop().unwrap();
            //dbg!("pushing to res:", curr.val);
            res.push(curr.val); // inherent copy for i32

            // place right node into stack first to maintain order after pop
            if let Some(right_child) = &curr.right {
                stack.push(right_child);
            }

            if let Some(left_child) = &curr.left {
                stack.push(left_child);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        //write test here
        let mut tree = Tree::new(10);

        let mut root = tree.root.unwrap();
        root.left = Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 40,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 50,
                left: None,
                right: None,
            })),
        }));
        root.right = Some(Box::new(TreeNode {
            val: 30,
            left: None,
            right: None,
        }));

        tree.root = Some(root);

        //println!("{:?}", tree);
        let data = tree.dfs();
        dbg!(data);
    }
}
