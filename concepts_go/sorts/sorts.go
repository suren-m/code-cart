package main

import (
	"fmt"
	"sort"
)

func main() {
	// Ints
	ints := []int{5, 2, 6, 3, 1, 4}
	sort.Ints(ints)
	fmt.Println(ints) // [1 2 3 4 5 6]

	// Float64s
	floats := []float64{5.2, 3.7, 6.5, 2.8, 4.1, 1.0}
	sort.Float64s(floats)
	fmt.Println(floats) // [1 2.8 3.7 4.1 5.2 6.5]

	// Strings
	strs := []string{"apple", "banana", "kiwi", "grape", "mango"}
	sort.Strings(strs)
	fmt.Println(strs) // [apple banana grape kiwi mango]

	// IntsAreSorted
	fmt.Println(sort.IntsAreSorted(ints)) // true

	// Float64sAreSorted
	fmt.Println(sort.Float64sAreSorted(floats)) // true

	// StringsAreSorted
	fmt.Println(sort.StringsAreSorted(strs)) // true

	// Reverse
	sort.Sort(sort.Reverse(sort.IntSlice(ints)))
	fmt.Println(ints) // [6 5 4 3 2 1]

	// Search
	index := sort.Search(len(ints), func(i int) bool { return ints[i] >= 3 })
	fmt.Println(index) // 3

	// SearchInts
	index = sort.SearchInts(ints, 3)
	fmt.Println(index) // 3

	// SearchFloat64s
	fIndex := sort.SearchFloat64s(floats, 4.1)
	fmt.Println(fIndex) // 3

	// SearchStrings
	sIndex := sort.SearchStrings(strs, "kiwi")
	fmt.Println(sIndex) // 2

	// Slice
	people := []struct {
		Name string
		Age  int
	}{
		{"Alice", 23},
		{"Eve", 2},
		{"Bob", 25},
	}
	sort.Slice(people, func(i, j int) bool {
		return people[i].Age < people[j].Age
	})
	fmt.Println(people) // [{Eve 2} {Alice 23} {Bob 25}]
}
