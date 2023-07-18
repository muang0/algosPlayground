package frequencysort

import "testing"

func TestFrequencySort(t *testing.T) {
	input := "Aabb"
	expected := "bbAa"
	actual := frequencySort(input)
	if actual != expected {
		t.Errorf("expected %v got %v", expected, actual)
	}
	input = "cccaaa"
	expected = "cccaaa"
	expectedtwo := "aaaccc"
	actual = frequencySort(input)
	if actual != expected && actual != expectedtwo {
		t.Errorf("expected %v got %v", expected, actual)
	}
	input = "tree"
	expected = "eert"
	expectedtwo = "eetr"
	actual = frequencySort(input)
	if actual != expected && actual != expectedtwo {
		t.Errorf("expected %v got %v", expected, actual)
	}
	input = "raaeaedere"
	expected = "eeeeaaarrd"
	actual = frequencySort(input)
	if actual != expected {
		t.Errorf("expected %v got %v", expected, actual)
	}
}
