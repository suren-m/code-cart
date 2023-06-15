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

	rules := map[int]string{
		3: "Fizz",
		5: "Buzz",
		7: "Fuzz",
	}

	order := []int{3, 5, 7}

	for i := 1; i <= 100; i++ {
		output := ""
		for _, k := range order {
			if i%k == 0 {
				output += rules[k]
			}
		}
		if output == "" {
			fmt.Println(i)
		} else {
			fmt.Println(output)
		}
	}
}
