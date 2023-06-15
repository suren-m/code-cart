// main.go

package main

import (
	"fmt"

	"structs/person/readonly"
)

func main() {
	p := readonly.NewPerson("Bob", 30)
	//fmt.Println(p.age) // private field
	fmt.Println(p.Name())
	fmt.Println(p.Age())
}
