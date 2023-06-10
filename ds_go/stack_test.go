package main

import (
	"fmt"
	"testing"
)

func init() {
	fmt.Println("Running Stack Tests")
}

func TestStack(t *testing.T) {
	stack := Stack{}
	stack.Push(10)
	stack.Push(20)
	stack.Push(30)

	_, err := stack.Pop()
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	printStackValues(&stack)

}

func printStackValues(stack *Stack) {
	fmt.Println(*stack)
}

// func TestMain(m *testing.M) {
// 	fmt.Println("Running stack tests...")
// 	m.Run()
// }
