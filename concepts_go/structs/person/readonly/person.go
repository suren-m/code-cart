package readonly

type Person struct {
	name string
	age  int
}

func NewPerson(name string, age int) *Person {
	return &Person{name: name, age: age}
}

func (p *Person) Name() string {
	return p.name
}

func (p *Person) Age() int {
	return p.age
}

// func ReadOnlyExample() {
// 	person := NewPerson("Alice", 30)
// 	fmt.Println(person.Name()) // Output: Alice
// 	fmt.Println(person.Age())  // Output: 30

// 	person.name = "Bob" // Modifying the field directly within the same package
// 	person.age = 35     // Modifying the field directly within the same package

// 	fmt.Println(person.Name()) // Output: Bob
// 	fmt.Println(person.Age())  // Output: 35
// }
