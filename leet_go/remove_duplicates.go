package main

// in-place.
// nums can be shrunk using the output to only contain distinct items
func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	currentIndex := 0

	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[currentIndex] {
			currentIndex++
			nums[currentIndex] = nums[i]
		}
	}

	return currentIndex + 1
}
