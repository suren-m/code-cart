package main

func isPalindrome(x int) bool {
	n := x
	rev := 0
	for n > 0 {	
		rem := n % 10
		rev = rev * 10 + rem
		n = n / 10
	}
	return x == rev
}