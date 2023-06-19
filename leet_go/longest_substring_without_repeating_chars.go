// https://leetcode.com/problems/longest-substring-without-repeating-characters/

package main

// substring not subsequence
// sliding window with two pointers
// move left (start) pointer when duplicate is found
func lengthOfLongestSubstring(s string) int {
	knownChars := make(map[rune]int, len(s))
	var start, end, max, currMax int = 0, 0, 0, 0

	for i, ch := range s {
		prevOccurence, chAlreadyExists := knownChars[ch]
		if chAlreadyExists {
			if start <= prevOccurence { // duplicate found if this is true
				currMax = end - start
				max = getMax(currMax, max)
				start = prevOccurence + 1 // new start (move left)
			}
		}
		end = end + 1
		knownChars[ch] = i
	}
	currMax = end - start // max may be found from last char. (e.g: pwwkew)
	return getMax(currMax, max)
}

func getMax(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}
