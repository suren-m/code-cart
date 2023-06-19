package main

// max diff between increasing elements
func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}
	max_diff, known_min := 0, prices[0]
	for _, v := range prices {
		if v < known_min {
			known_min = v
		} else if v - known_min > max_diff {
			max_diff = v - known_min
		}
	}
	return max_diff
}