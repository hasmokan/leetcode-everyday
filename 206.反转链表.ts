/*
 * @lc app=leetcode.cn id=206 lang=typescript
 *
 * [206] 反转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function reverseList(head: ListNode | null): ListNode | null {
    if (!head) return head

    let cur = head
    let pre = null

    let dimmy = null

    while (cur && cur.next) {
        let tmp = cur.next
        cur.next = pre;
        pre = cur;
        cur = tmp;
    }

    cur.next = pre;

    return cur
};
// @lc code=end

