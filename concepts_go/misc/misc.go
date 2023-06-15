package main

import "fmt"

func fizzbuzzfuzz(n int) {
	rules := map[int]string{
		3: "Fizz",
		5: "Buzz",
		7: "Fuzz",
	}

	order := []int{3, 5, 7}

	for i := 1; i <= n; i++ {
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

func main() {
	fizzbuzzfuzz(100)
}
