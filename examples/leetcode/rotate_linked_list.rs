#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

// 1 -> 2 -> 3 -> 4 -> 5, k = 2
// ===
// 4 -> 5 -> 1 -> 2 -> 3
// algo: iterate over linked list, count it, k = k % n
// traverse from head to pivot, it is n - k - 1 iters, lets name it 3
// - result = 3.next,
// - 3.next = None,
// tail.next = head
// - return result
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }

        // step 1: count n
        let mut n: i32 = 0;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_deref();
        }

        let k = k % n;
        if k == 0 { return head; }

        // step 2: walk to pivot
        let mut new_head = {
            let mut pivot = head.as_deref_mut().unwrap();
            for _ in 0..(n - k - 1) {
                pivot = pivot.next.as_deref_mut().unwrap();
            }
            pivot.next.take()
        };

        // step 3: find tail of new_head, attach old head
        let mut new_tail = new_head.as_deref_mut().unwrap();
        while new_tail.next.is_some() {
            new_tail = new_tail.next.as_deref_mut().unwrap();
        }
        new_tail.next = head;

        new_head
    }
}

fn main() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5]);
    let rotated_list = Solution::rotate_right(head, 2);
    let result = list_to_vec(rotated_list);
    println!("{:?}", result);
}

fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in v.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut current = list;
    loop {
        if let Some(n) = current {
            result.push(n.val);
            current = n.next;
        } else {
            break result;
        }
    }
}