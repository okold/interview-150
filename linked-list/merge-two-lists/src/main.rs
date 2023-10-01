// Definition for singly-linked list.
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

fn main() {
  pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge_helper(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = merge_helper(l1.next.take(), Some(l2));
                    Some(l1)
                } else {
                    l2.next = merge_helper(Some(l1), l2.next.take());
                    Some(l2)
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
        }
    }

    merge_helper(list1, list2)
  }

  fn print_list(node: Option<Box<ListNode>>) {
    match node {
      Some(x) => {
        print!("{}", x.val);
        print_list(x.next);
      },
      None => println!(),
    }
  }

  // i really don't get why they didn't use references
  // creating a new list like this feels like garbage lol
  let a = Some(Box::new(ListNode {
    val: (1),
    next: Some(Box::new(ListNode { 
      val: (2), 
      next: Some(Box::new(ListNode::new(4))) }))
  }));

  let b = Some(Box::new(ListNode {
    val: (1),
    next: Some(Box::new(ListNode { 
      val: (3), 
      next: Some(Box::new(ListNode::new(4))) }))
  }));

  //print_list(a);
  //print_list(b);

  let c = merge_two_lists(a, b);
  print_list(c);
}