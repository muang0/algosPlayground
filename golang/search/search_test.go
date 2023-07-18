package search

import "testing"

func TestLinearSearch(t *testing.T) {
	input := []int{3, 5, 7, 11, 15, 27}
	result, _ := linearsearch(input, 15)
	expected := 4
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
}

func TestBinarySearch(t *testing.T) {
	input := []int{3, 5, 7, 11, 15, 27}
	result, err := binarysearch(input, 3, 0, 5)
	expected := 0
	if result != expected && err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	result, _ = binarysearch(input, 15, 0, 5)
	expected = 4
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	_, err = binarysearch(input, -1, 0, 5)
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
}

func TestJumpSearch(t *testing.T) {
	input := []int{3, 5, 7, 11, 15, 27}
	result, err := jumpsearch(input, 3, 2)
	expected := 0
	if result != expected && err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	result, _ = jumpsearch(input, 15, 1)
	expected = 4
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	result, _ = jumpsearch(input, 15, 100)
	expected = 4
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	_, err = jumpsearch(input, 15, -1)
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
	_, err = jumpsearch(input, -1, 3)
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
}

func TestInterpolationSearch(t *testing.T) {
	input := []int{3, 6, 9, 12, 15, 18}
	result, err := interpolationsearch(input, 12, 0, 4)
	expected := 3
	if result != expected || err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	result, err = interpolationsearch(input, 3, 0, 4)
	expected = 0
	if result != expected || err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	_, err = interpolationsearch(input, 1, 0, 4)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
	_, err = interpolationsearch(input, 10, 0, 4)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
}

func TestExponentialSearch(t *testing.T) {
	input := []int{3, 6, 9, 12, 15, 18}
	result, err := exponentailsearch(input, 12)
	expected := 3
	if result != expected || err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	result, err = exponentailsearch(input, 3)
	expected = 0
	if result != expected || err != nil {
		t.Errorf("expected %v got %v", expected, result)
	}
	_, err = exponentailsearch(input, 1)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
	_, err = exponentailsearch(input, 10)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
	_, err = exponentailsearch(input, -10)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
	_, err = exponentailsearch(input, 13330)
	expected = 0
	if err == nil {
		t.Errorf("error should have been thrown for element not in slice")
	}
}
