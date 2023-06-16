package main

import (
	"fmt"
)

func main() {
	fmt.Println("...")

	x := []int{1, 2, 3, 4}
	for c := range x {
		fmt.Println(c)
	}

	y := map[string]int{}
	y["a"] = 1
	y["b"] = 2
	y["c"] = 3
	for k, v := range y {
		fmt.Println(k, v)
	}

	z := make([]int, 5)
	fmt.Println(z, len(z), cap(z))
}
