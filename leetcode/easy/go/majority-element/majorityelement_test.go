package majorityelement

import "testing"

func TestMajorityElement(t *testing.T) {
	input := []int{3, 2, 3, 3, 3, 3, 8, 8, 8}
	result := majorityElement(input)
	expected := 3
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
}
