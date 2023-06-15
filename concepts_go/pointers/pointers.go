package main

import (
	"fmt"
)

func main() {

	strings()
	doublePointer()

}

func strings() {
	s := "hello"
	s1 := s // new allocation
	s2 := &s1
	*s2 = "world"
	fmt.Println("..Strings..")
	fmt.Printf("s: %s, s1: %s \n", s, s1)
	fmt.Printf("&s:%p, &s1: %p, &s2: %p \n", &s, &s1, &s2)
}

func doublePointer() {
	s := "hello"
	s1 := &s
	s2 := &s1
	**s2 = "world"
	fmt.Println("..double pointer with strings..")
	fmt.Printf("s: %s \n", s)
	fmt.Printf("&s:%p, &s1: %p, &s2: %p \n", &s, &s1, &s2)
}
