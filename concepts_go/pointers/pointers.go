package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {

	strings()
	doublePointer()

	fmt.Println("Simple Linked List")
	head := ListNode{Val: 1, Next: nil}
	second := ListNode{Val: 2, Next: nil}
	third := ListNode{Val: 3, Next: nil}
	second.Next = &third
	head.Next = &second

	node := &head
	i := 1
	for node != nil {
		fmt.Printf("Node %d: %v \n", i, node.Val)
		node = node.Next
	}
}

func strings() {
	s := "hello"
	s1 := s // new allocation
	s2 := &s1
	*s2 = "world"
	fmt.Println("..Strings..")
	fmt.Printf("s: %s, s1: %s \n", s, s1)
	fmt.Printf("&s:%p, &s1: %p, &s2: %p \n", &s, &s1, &s2)
}

func doublePointer() {
	s := "hello"
	s1 := &s
	s2 := &s1
	**s2 = "world"
	fmt.Println("..double pointer with strings..")
	fmt.Printf("s: %s \n", s)
	fmt.Printf("&s:%p, &s1: %p, &s2: %p \n", &s, &s1, &s2)
}
