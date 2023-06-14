package main

import (
	"fmt"
	"math"
)

type Shape interface {
	Area() float64
}

type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

type Rectangle struct {
	Width  float64
	Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func printArea(s Shape) {
	switch shape := s.(type) {
	case Circle:
		fmt.Printf("Circle with radius %.2f has area %.2f\n", shape.Radius, shape.Area())
	case Rectangle:
		fmt.Printf("Rectangle with width %.2f and height %.2f has area %.2f\n", shape.Width, shape.Height, shape.Area())
	default:
		fmt.Println("Unknown shape")
	}
}

// Composite
type Animal interface {
	MakeSound()
}

type Dog interface {
	Animal
	WagTail()
}

type MyDog struct{}

func (d MyDog) MakeSound() {
	fmt.Println("Woof!")
}

func (d MyDog) WagTail() {
	fmt.Println("Wagging Tail")
}

func main() {
	var c Shape = Circle{Radius: 3}
	//	c := Circle { Radius: 3 }
	printArea(c)
	var myDog Dog = MyDog{}
	//myDog.MakeSound()
	myDog.WagTail()
}
