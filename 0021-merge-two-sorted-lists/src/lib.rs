// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match list1 {
            Some(head1) => match list2 {
                Some(head2) => {
                    let mut merged_list;

                    if head1.as_ref().val < head2.as_ref().val {
                        merged_list = head1;
                        list1 = head1.next;

                        while let Some(head) = list1 {
                            if head.as_ref().val < head2.as_ref().val {
                                merged_list.next = Some(head);
                                list1 = head.next;
                            }
                        }
                    } else {
                        merged_list = head2;
                        list2 = head2.next;

                        // while let Some(head2) = head2.as_ref().next {
                        //     if head1.as_ref().val > head2.as_ref().val {
                        //         merged_list.unwrap().as_mut().next = Some(head2)
                        //     }
                        // }
                    }

                    Some(merged_list)
                }
                None => return Some(head1),
            },
            None => match list2 {
                Some(head2) => return Some(head2),
                None => return None::<Box<ListNode>>,
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let expected_list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), expected_list);
    }
}
