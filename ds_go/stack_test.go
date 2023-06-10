package main

import (
	"fmt"
	"testing"
)

func TestStack(t *testing.T) {
	stack := Stack{}
	stack.Push(10)
	stack.Push(20)
	stack.Push(30)

	pop, err := stack.Pop()
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	fmt.Printf("Popped item: %d \n", pop)

	fmt.Println("Stack values:")
	printStackValues(&stack)

	fmt.Println("Is stack empty?", stack.IsEmpty())
}

func printStackValues(stack *Stack) {
	fmt.Println(*stack)
}

func TestMain(m *testing.M) {
	fmt.Println("Running stack tests...")
	m.Run()
}
