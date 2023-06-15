package main

import "fmt"

const (
	Sunday = iota
	Monday
	Tuesday
)

var days = []string{"sun", "mon", "tue"}

func main() {
	fmt.Println(days[Sunday], days[Monday], Tuesday)

}
