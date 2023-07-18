package removedups

import (
	"testing"
)

func TestRemoveDuplicates(t *testing.T) {
	input := []int{3, 4, 5}
	result := removeDuplicates(input)
	expected := 3
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	input = []int{3, 3, 4, 5, 5}
	result = removeDuplicates(input)
	expected = 3
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	input = []int{-3, 1, 3, 3, 4, 5, 5, 11, 11, 11}
	result = removeDuplicates(input)
	expected = 6
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
}
