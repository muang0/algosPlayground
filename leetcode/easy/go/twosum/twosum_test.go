package twosum

import (
	"sort"
	"testing"
)

func TestTwoSum(t *testing.T) {
	nums := []int{2, 7, 11, 15}
	target := 13
	ans1 := []int{0, 2} // increasing numeric order
	ans := twosum(nums, target)
	sort.Ints(ans)
	if ans == nil {
		t.Errorf("expecting %v got %v", ans1, ans)
	}
	for i, n := range ans {
		if n != ans1[i] {
			t.Errorf("expecting %v got %v", ans1, ans)
		}
	}
}
