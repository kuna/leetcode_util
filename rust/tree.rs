use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn gen_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.len() == 0 {
        return None;
    }
    let mut q = VecDeque::from([Some(Rc::new(RefCell::new(TreeNode::new(arr[0]))))]);

    // Clone is acceptable as we're just increasing RefCount
    let root = match &q[0] {
        Some(x) => Some(Rc::clone(x)),
        _ => unreachable!(),
    };

    // Now we dig into queue and input array to build the tree
    let mut aiter = arr.iter().peekable();
    aiter.next(); // consume first one as it's already used
    while !q.is_empty() && aiter.peek().is_some() {
        let width_len = q.len();
        (0..width_len).for_each(|_| {
            let node = q.pop_front().unwrap();
            match &node {
                Some(x) => {
                    let left = if let Some(n) = aiter.next() {
                        if *n >= 0 {
                            Some(Rc::new(RefCell::new(TreeNode::new(*n))))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let right = if let Some(n) = aiter.next() {
                        if *n >= 0 {
                            Some(Rc::new(RefCell::new(TreeNode::new(*n))))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    // clone is okay as it's RC
                    q.push_back(left.clone());
                    q.push_back(right.clone());
                    let mut xmut = x.borrow_mut();
                    xmut.left = left;
                    xmut.right = right;
                }
                _ => {
                    // just skip two arr inputs
                    aiter.next();
                    aiter.next();
                    q.push_back(None);
                    q.push_back(None);
                }
            }
        });
    }

    root
}

fn print_tree(t: Option<Rc<RefCell<TreeNode>>>) {
    let mut v = VecDeque::from([t]);
    while !v.is_empty() {
        let curr_len = v.len();
        let mut alive_elem_cnt = 0;
        let curr_level = (0..curr_len)
            .map(|_| {
                match v.pop_front() {
                    Some(Some(n)) => {
                        alive_elem_cnt += 1;
                        let n = n.borrow();
                        // Add all child in the queue
                        // Clone is cost-safe as RC only increases RefCount
                        v.push_back(match &n.left {
                            Some(x) => Some(Rc::clone(&x)),
                            _ => None,
                        });
                        v.push_back(match &n.right {
                            Some(x) => Some(Rc::clone(&x)),
                            _ => None,
                        });
                        n.val.to_string()
                    }
                    _ => {
                        v.push_back(None);
                        v.push_back(None);
                        ".".to_string()
                    }
                }
            })
            .collect::<Vec<_>>();
        if alive_elem_cnt == 0 {
            break;
        }
        println!("ㅏ {:?}", curr_level);
    }
}

fn main() {
    // empty tree
    let sample_tree = gen_tree(&[]);
    print_tree(sample_tree);
    println!("---");

    // full binary tree
    let sample_tree = gen_tree(&[1, 2, 3, 4, 5, 6, 7]);
    print_tree(sample_tree);
    println!("---");

    // plain, sparse binary tree
    let sample_tree = gen_tree(&[1, -1, 2, -1, -1, 3]);
    print_tree(sample_tree);
    println!("---");

    println!("Hello, world!");
}
