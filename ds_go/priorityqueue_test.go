package main

import (
	"container/heap"
	"fmt"
	"testing"
)
func init() {
	fmt.Println("Running PriorityQueue Tests")
}
func TestPush(t *testing.T) {
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)

	item := &Item{
		value:    "orange",
		priority: 1,
	}
	heap.Push(&pq, item)

	if pq.Len() != 1 {
		t.Errorf("Push failed, expected length 1, got %d", pq.Len())
	}
}

func TestPop(t *testing.T) {
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)

	item := &Item{
		value:    "orange",
		priority: 1,
	}
	heap.Push(&pq, item)
	popped := heap.Pop(&pq).(*Item)

	if popped != item {
		t.Errorf("Pop failed, expected %v, got %v", item, popped)
	}
}

func TestUpdate(t *testing.T) {
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)

	item := &Item{
		value:    "orange",
		priority: 1,
	}
	heap.Push(&pq, item)
	pq.update(item, "grape", 2)

	if item.value != "grape" || item.priority != 2 {
		t.Errorf("Update failed, expected value 'grape' and priority 2, got value '%s' and priority %d", item.value, item.priority)
	}
}

func TestPriorityOrder(t *testing.T) {
	items := map[string]int{
		"banana": 3, "apple": 2, "pear": 4,
	}
	pq := make(PriorityQueue, len(items))

	i := 0
	for value, priority := range items {
		pq[i] = &Item{
			value:    value,
			priority: priority,
			index:    i,
		}
		i++
	}
	heap.Init(&pq)

	// Insert a new item and then modify its priority.
	item := &Item{
		value:    "orange",
		priority: 1,
	}
	heap.Push(&pq, item)
	pq.update(item, item.value, 5)

	// Test Pop to give us items in decreasing priority order.
	expectedOrder := []string{"orange", "pear", "banana", "apple"}

	for _, value := range expectedOrder {
		if pq.Len() == 0 {
			t.Errorf("Pop failed, expected to pop %s, but queue is empty", value)
		}

		item := heap.Pop(&pq).(*Item)
		if item.value != value {
			t.Errorf("Pop failed, expected to pop %s, got %s", value, item.value)
		}
	}
}
