package removedups

// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

// constraint for problem was to replace in slice-- minimize memory as much as possible
// slow runtime likely due to most submissions focusing on runtime rather than mem
// Runtime: 24 ms, faster than 8.54% of Go online submissions for Remove Duplicates from Sorted Array.
// Memory Usage: 4.6 MB, less than 100.00% of Go online submissions for Remove Duplicates from Sorted Array.

func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	} else if len(nums) == 1 {
		return 1
	}
	for i := 1; i < len(nums); i++ {
		if nums[i] == nums[i-1] {
			copy(nums[i:], nums[i+1:])
			nums = nums[:len(nums)-1]
			i--
		}
	}
	return len(nums)
}
