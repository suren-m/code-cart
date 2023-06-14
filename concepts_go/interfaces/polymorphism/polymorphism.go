package main

import (
	"fmt"
	"math"
)

// Shape interface
type Shape interface {
	Area() float64
}

// Circle struct
type Circle struct {
	radius float64
}

// Area method for Circle
func (c Circle) Area() float64 {
	return math.Pi * c.radius * c.radius
}

// Rectangle struct
type Rectangle struct {
	width, height float64
}

// Area method for Rectangle
func (r Rectangle) Area() float64 {
	return r.width * r.height
}

// CalculateTotalArea takes multiple shapes and calculates total area
func CalculateTotalArea(shapes ...Shape) float64 {
	var totalArea float64
	for _, shape := range shapes {
		totalArea += shape.Area()
	}
	return totalArea
}

func main() {
	c := Circle{radius: 5}
	r := Rectangle{width: 3, height: 4}

	// Using CalculateTotalArea function
	total := CalculateTotalArea(c, r)
	fmt.Printf("Total area: %f\n", total)
}
