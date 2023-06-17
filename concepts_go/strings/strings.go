package main

import (
	"fmt"
	"strings"
	"unicode"
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

func stringFunctions() {

	// Compare
	fmt.Println(strings.Compare("a", "b")) // -1

	// Contains
	fmt.Println(strings.Contains("test", "es")) // true

	// ContainsAny
	fmt.Println(strings.ContainsAny("team", "i")) // false

	// ContainsRune
	fmt.Println(strings.ContainsRune("gopher", 'r')) // true

	// Count
	fmt.Println(strings.Count("cheese", "e")) // 3

	// EqualFold
	fmt.Println(strings.EqualFold("Go", "go")) // true

	// Fields
	fmt.Println(strings.Fields("  foo bar  baz   ")) // [foo bar baz]

	// HasPrefix
	fmt.Println(strings.HasPrefix("Gopher", "Go")) // true

	// HasSuffix
	fmt.Println(strings.HasSuffix("Amigo", "go")) // true

	// Index
	fmt.Println(strings.Index("chicken", "ken")) // 4

	// IndexAny
	fmt.Println(strings.IndexAny("chicken", "aeiouy")) // 2

	// IndexByte
	fmt.Println(strings.IndexByte("golang", 'g')) // 0

	// IndexFunc
	f := func(c rune) bool {
		return c < 'A'
	}
	fmt.Println(strings.IndexFunc("Hello, 世界", f)) // 5

	// IndexRune
	fmt.Println(strings.IndexRune("gopher", 'r')) // 4

	// Join
	s := []string{"foo", "bar", "baz"}
	fmt.Println(strings.Join(s, ", ")) // foo, bar, baz

	// LastIndex
	fmt.Println(strings.LastIndex("go gopher", "go")) // 3

	// LastIndexAny
	fmt.Println(strings.LastIndexAny("go gopher", "go")) // 4

	// LastIndexByte
	fmt.Println(strings.LastIndexByte("golang", 'g')) // 5

	// LastIndexFunc
	fmt.Println(strings.LastIndexFunc("go 123!", f)) // 5

	// Map
	mapper := func(r rune) rune {
		return r + 1
	}
	fmt.Println(strings.Map(mapper, "HAL-9000")) // IBM.:111

	// Repeat
	fmt.Println(strings.Repeat("na", 3)) // nanana

	// Replace
	fmt.Println(strings.Replace("oink oink oink", "k", "ky", 2)) // oinky oinky oink

	// ReplaceAll
	fmt.Println(strings.ReplaceAll("oink oink oink", "oink", "moo")) // moo moo moo

	// Split
	fmt.Println(strings.Split("a,b,c", ",")) // [a b c]

	// SplitAfter
	fmt.Println(strings.SplitAfter("a,b,c", ",")) // [a, b, c,]

	// SplitAfterN
	fmt.Println(strings.SplitAfterN("a,b,c", ",", 2)) // [a, b,c]

	// SplitN
	fmt.Println(strings.SplitN("a,b,c", ",", 2)) // [a b,c]

	// ToLower
	fmt.Println(strings.ToLower("Gopher")) // gopher

	// ToTitle
	fmt.Println(strings.ToTitle("loud noises")) // LOUD NOISES

	// ToUpper
	fmt.Println(strings.ToUpper("Gopher")) // GOPHER

	// Trim
	fmt.Println(strings.Trim("¡¡¡Hello, Gophers!!!", "!¡")) // Hello, Gophers

	// TrimFunc
	fmt.Println(strings.TrimFunc("¡¡¡Hello, Gophers!!!", func(r rune) bool { return !unicode.IsLetter(r) })) // Hello, Gophers

	// TrimLeft
	fmt.Println(strings.TrimLeft("¡¡¡Hello, Gophers!!!", "!¡")) // Hello, Gophers!!!

	// TrimLeftFunc
	fmt.Println(strings.TrimLeftFunc("¡¡¡Hello, Gophers!!!", func(r rune) bool { return !unicode.IsLetter(r) })) // Hello, Gophers!!!

	// TrimPrefix
	fmt.Println(strings.TrimPrefix("Goodbye, world!", "Goodbye, ")) // world!

	// TrimRight
	fmt.Println(strings.TrimRight("¡¡¡Hello, Gophers!!!", "!¡")) // ¡¡¡Hello, Gophers

	// TrimRightFunc
	fmt.Println(strings.TrimRightFunc("¡¡¡Hello, Gophers!!!", func(r rune) bool { return !unicode.IsLetter(r) })) // ¡¡¡Hello, Gophers

	// TrimSpace
	fmt.Println(strings.TrimSpace(" \t\n Hello, Gophers \n\t\r\n")) // Hello, Gophers

	// TrimSuffix
	fmt.Println(strings.TrimSuffix("Hello, world!", ", world!")) // Hello
}
