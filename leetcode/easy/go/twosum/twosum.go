package twosum

// https://leetcode.com/problems/two-sum/

// V2 use map data struct
// solution: Runtime: 4 ms, faster than 92.23% of Go online submissions for Two Sum.
// Memory Usage: 3.8 MB, less than 11.54% of Go online submissions for Two Sum.

func twosum(nums []int, target int) []int {
	lnums := len(nums)
	summap := map[int]int{}
	for i1 := 0; i1 < lnums; i1++ {
		if val, ok := summap[target-nums[i1]]; ok {
			return []int{val, i1}
		}
		summap[nums[i1]] = i1
	}
	return nil
}

// V1
// solution: Runtime: 36 ms, faster than 30.36% of Go online submissions for Two Sum.
//	Memory Usage: 2.9 MB, less than 100.00% of Go online submissions for Two Sum.
// func twosum(nums []int, target int) []int {
// 	lnums := len(nums)
// 	for i1 := 0; i1 < lnums; i1++ {
// 		for i2 := i1 + 1; i2 < lnums; i2++ {
// 			if nums[i1]+nums[i2] == target {
// 				return []int{i1, i2}
// 			}
// 		}
// 	}
// 	return nil
// }
