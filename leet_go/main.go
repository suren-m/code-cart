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
	fmt.Println("val:", y["c"])
	val, ok := y["d"]
	if ok {
		fmt.Println("val not exist")
	} else {
		fmt.Println("val:", val)
	}
	for k, v := range y {
		fmt.Println(k, v)
	}
	fmt.Println("len:", len(y))

	z := make([]int, 5)
	fmt.Println(z, len(z), cap(z))

	fmt.Println("dupes removed:", removeDuplicates([]int{1,1,2}))
}
