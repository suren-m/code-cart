package main

import "fmt"

// Variadic function that accepts any number of integers.
func sum(nums ...int) int {
	fmt.Print(nums, " ")
	total := 0
	for _, num := range nums {
		total += num
	}
	return total
}

func variadicParams() {
	// You can call a variadic function with multiple separate arguments:
	fmt.Println(sum(1, 2, 3))

	// You can also pass a slice to a variadic function using the ... operator:
	nums := []int{1, 2, 3, 4}
	fmt.Println(sum(nums...))

	// You can pass multiple slices to the function, but you need to combine them into a single slice first:
	nums1 := []int{1, 2, 3}
	nums2 := []int{4, 5, 6}
	nums4 := append(nums1, nums2...)
	fmt.Println(sum(nums4...))

	// You can pass a combination of a slice and separate elements,
	// but you need to append the separate elements to the slice first:
	nums5 := []int{1, 2, 3}
	nums5 = append(nums5, 4, 5)
	fmt.Println(sum(nums5...))
}
