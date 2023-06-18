package main

import (
	"fmt"
	"time"
)

func producer(ch chan<- int) {
	for i := 1; i <= 5; i++ {
		ch <- i
		time.Sleep(500 * time.Millisecond)
	}
	close(ch)
}

func consumer(ch <-chan int) {
	for value := range ch {
		fmt.Println("Received:", value)
		time.Sleep(1 * time.Second)
	}
}

func main() {
	ch := make(chan int)

	go producer(ch)
	consumer(ch)

	fmt.Println("Program completed")
}
