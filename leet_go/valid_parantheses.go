package main

func isValid(s string) bool {
	if len(s)%2 == 1 { // odd length is always invalid
		return false
	}

	stack := make([]rune, 0, len(s))
	for _, c := range s {
		switch c {
		case '(':
			stack = append(stack, ')')
		case '{':
			stack = append(stack, '}')
		case '[':
			stack = append(stack, ']')
		default:
			// can't have closing bracket without opening bracket 
			// or closing bracket must match the last bracket added in previous iteration
			if len(stack) == 0 || stack[len(stack)-1] != c {
				return false
			}
			stack = stack[:len(stack)-1] // pop and shrink the stack
		}
	}

	return len(stack) == 0
}

func isValid_v2(s string) bool {
	if len(s) % 2 != 0 {
		return false
	}

	stack := make([]int, len(s))
	for ch := range s {
		if ch == '(' || ch == '[' || ch == '{' {
			stack = append(stack, ch)
		} 
	}
	return true
}
