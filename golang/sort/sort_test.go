package sort

import (
	"testing"
)

// helper function for testing slice equality
// https://stackoverflow.com/questions/15311969/checking-the-equality-of-two-slices
func testEq(a []int, b []int) bool {
	// If one is nil, the other must also be nil.
	if (a == nil) != (b == nil) {
		return false
	}
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
func TestSelectionSort(t *testing.T) {
	input := []int{18, 12, 3, 15, 6, 9}
	result, _ := selectionsort(input)
	expected := []int{3, 6, 9, 12, 15, 18}
	if testEq(expected, result) != true {
		t.Errorf("expected %v got %v", expected, result)
	}
	input = []int{}
	result, _ = selectionsort(input)
	expected = []int{}
	if testEq(expected, result) != true {
		t.Errorf("expected %v got %v", expected, result)
	}
	input = []int{-2, -3333, 2222, 123}
	result, _ = selectionsort(input)
	expected = []int{-3333, -2, 123, 2222}
	if testEq(expected, result) != true {
		t.Errorf("expected %v got %v", expected, result)
	}
}
