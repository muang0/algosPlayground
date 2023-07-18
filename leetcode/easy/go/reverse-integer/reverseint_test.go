package reverseint

import (
	"testing"
)

func TestReverseInt(t *testing.T) {
	result := reverse(-4324)
	expected := -4234
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	result = reverse(-4322342444444)
	expected = 0
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	result = reverse(12345)
	expected = 54321
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	result = reverse(-42933)
	expected = -33924
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
}
