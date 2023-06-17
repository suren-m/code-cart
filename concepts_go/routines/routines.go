package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

func main() {
	
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
	for i :=1; i<=5; i++ {
		wg.Add(1)
		go x(i)
	}
	wg.Wait()
}