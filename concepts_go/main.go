// Learning Go from Rust
// Crates ~ Packages, Use ~ import
// Rust: fn add(x: i32, y: i32) -> i32 {}
// Go: func add(x,y int) int {}
// Rust: match scrutinee_exp { arm => exp, arm => exp, _ => exp }
// Go: switch exp { case arm: exp, default: exp }
// Arrays:
//
//	x.len()
//	len(x)
package main

import (
	"fmt"
	"time"
	_ "rsc.io/quote"
)

func main() {
	p(add(2, 2))
	switching()

}

func p(i interface{}) {
	fmt.Println(i)
}

func add(x, y int) int {
	return x + y
}

func switching() {

	t := time.Now().UTC().Weekday()
	switch t {
	case time.Saturday, time.Monday:
		fmt.Println("Weekend")
	case time.Friday:
		fmt.Println("Almost Weekend")
	default:
		fmt.Println("Weekday")
	}

	whatAmI := func(i interface{}) {
		switch t := i.(type) {
		case int:
			p("number")
		case bool:
			p("boolean")
		default:
			fmt.Printf("other %T \n", t)
		}
	}
	// has to be after the func variable declaration
	whatAmI(10)
	whatAmI("hello")
}
