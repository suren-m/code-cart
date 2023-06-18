package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

func main() {
	//maps()
	fmt.Println("main done")
}

func wg() {
	var wg sync.WaitGroup
	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	x := func(i int) {
		defer wg.Done()
		sleepTime := r.Intn(5)
		fmt.Println(i, "sleeping for", sleepTime, "seconds")
		time.Sleep((time.Duration(sleepTime) * time.Second))
		fmt.Println(i, "routine complete")
	}
	for i := 1; i <= 5; i++ {
		wg.Add(1)
		go x(i)
	}
	wg.Wait()
}

func maps() {
	// Map example
	var myMap sync.Map
	myMap.Store("key1", "value1")
	value, found := myMap.Load("key1")
	if found {
		fmt.Println(value) // Output: value1
	}
}

func mutex() {
	// Mutex example
	var mutex sync.Mutex
	counter := 0

	go func() {
		mutex.Lock()
		counter++
		mutex.Unlock()
	}()

	mutex.Lock()
	counter++
	mutex.Unlock()

	fmt.Println(counter) // Output: 2

}
