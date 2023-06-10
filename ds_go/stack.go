package main

import (
	"fmt"
)

type Stack []int

func (s *Stack) Push(v int) {
	*s = append(*s, v)
}

func (s *Stack) Pop() (int, error) {
	if len(*s) == 0 {
		return 0, fmt.Errorf("Stack is empty")
	}

	last := (*s)[len(*s)-1]
	*s = (*s)[:len(*s)-1]

	return last, nil
}

func (s *Stack) IsEmpty() bool {
	return len(*s) == 0
}
