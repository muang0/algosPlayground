package reverseint

import (
	"math"
)

// https://leetcode.com/problems/reverse-integer

// Runtime: 4 ms, faster than 38.72% of Go online submissions for Reverse Integer.
// Memory Usage: 2.2 MB, less than 80.00% of Go online submissions for Reverse Integer.

func reverse(x int) int {
	var rval float64 = 0
	// isolate each digit by using modulo operator & append to rval
	for i := 10; i > 0; i = i * 10 {
		rval = (10 * rval) + math.Floor(float64((x%i)/(i/10)))
		if x/i == 0 {
			break
		}
	}
	// account for 'overflow'
	if rval > (math.Pow(2, 31)-1) || rval < -(math.Pow(2, 31)) {
		return 0
	}
	return int(rval)
}
