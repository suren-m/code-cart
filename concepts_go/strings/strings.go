package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	c := 'a' // rune - alias for int32
	fmt.Printf("c: %c v: %v T: %T\n", c, c, c)
	v := []int{'h', 'e', 'l', 'l', '🙂'}
	fmt.Printf("v: %v c: %c\n", v[len(v)-2:], v[len(v)-2:])

	name := "bob"
	greet := fmt.Sprint("Sprint: Hello ", name)
	fmt.Println(greet)

	name2 := "alice"
	greet2 := fmt.Sprintf("Sprintf: Hello %s", name2)
	fmt.Println(greet2)

	// Byte arrays
	s := "hello 👻"
	var b []byte = []byte(s)
	fmt.Printf("b []byte: %v *b: %p *s: %p \n", b, &b, &s)
	fmt.Println("[]byte to string: ", string(b))

	fmt.Println("..Loop through []byte and utf8.DecodeRune")
	for len(b) > 0 {
		r, size := utf8.DecodeRune(b)
		fmt.Printf("%c ", r)
		b = b[size:]
		if len(b) == 0 {
			fmt.Printf("\n")
		}
	}
	b = utf8.AppendRune(b, '🙂')
	fmt.Printf("AppendRune: %s \n", b)

	b = append(b, make([]byte, 4)...)
	utf8.EncodeRune(b[len(b)-4:], '😀')
	fmt.Printf("Encode after Append: %s \n", b)

}
