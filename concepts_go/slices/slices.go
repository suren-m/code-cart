package main

import "fmt"

func main() {
	x := []int{1, 2, 3, 4, 5}
	y := []int{4, 5, 6}
	y = append(x, y...)
	msg := fmt.Sprintf(" x: %v (%p) \n y: %v (%p) \n", x, &x, y, &y)
	fmt.Print(msg)

	i := 0
	j := len(x) - 1
	for i <= j {
		fmt.Printf("x[%v]: %v x[%v]: %v \n", i, x[i], j, x[j])
		i++
		j--
	}
	twoDim()
}

func twoDim() {
	a := [][]int { {1, 2, 3}, {4,5,6}, {7,8,9}}
	fmt.Printf("%v\n", a[0][1])
}