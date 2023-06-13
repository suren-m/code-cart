package main

func twoSum(nums []int, target int) []int {
    cache := make(map[int]int) 

    for i, n := range nums {
        dist := target - n
        val, exists := cache[dist] 
		if exists {
			return []int{val, i}
		} else {
			cache[n] = i
		}
    }
	return []int{}
}