package majorityelement

import (
	"fmt"
	"os"
)

// https://leetcode.com/problems/majority-element/

// Runtime: 20 ms, faster than 49.21% of Go online submissions for Majority Element.
//   faster solution using sorting then return median int ?
// Memory Usage: 6 MB, less than 50.00% of Go online submissions for Majority Element.

func majorityElement(nums []int) int {
	// hashmap solution, return as soon as majority element found
	nummap := map[int]int{}
	mincount := len(nums) / 2
	for _, num := range nums {
		if c, ok := nummap[num]; ok {
			nummap[num] = c + 1
		} else {
			nummap[num] = 1
		}
		if nummap[num] > mincount {
			return num
		}
	}
	fmt.Println("Error: majority element not found in string")
	os.Exit(1)
	return 0
}
