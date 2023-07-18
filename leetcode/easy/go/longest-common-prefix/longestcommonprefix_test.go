package longestcommprefix

import (
	"testing"
)

func TestLcp(t *testing.T) {
	input := make([]string, 3)
	input[0] = "flower"
	input[1] = "flow"
	input[2] = "flight"
	output := lcp(input)
	expectedout := "fl"
	if output != expectedout {
		t.Errorf("expected %v got %v", expectedout, output)
	}
}
