package main

import (
	"fmt"
	"reflect"
	"testing"
)

func TestRemoveDuplicates(t *testing.T) {
	testCases := []struct {
		input  []int
		output int
		result []int
	}{
		{[]int{1, 1, 2}, 2, []int{1, 2}},
		{[]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, 5, []int{0, 1, 2, 3, 4}},
		{[]int{1, 1, 1, 1, 1}, 1, []int{1}},
		{[]int{}, 0, []int{}}, // edge case of empty input
	}

	for _, testCase := range testCases {
		fmt.Printf("Testing case with input: %v\n", testCase.input)
		numElements := removeDuplicates(testCase.input)
		if numElements != testCase.output {
			t.Errorf("Expected numElements=%v but got %v", testCase.output, numElements)
		}
		// check that the first n elements in the slice match the expected results
		if !reflect.DeepEqual(testCase.input[:numElements], testCase.result) {
			t.Errorf("Expected result=%v but got %v", testCase.result, testCase.input[:numElements])
		}
		fmt.Printf("Passed for input: %v\n", testCase.input)
	}
}
