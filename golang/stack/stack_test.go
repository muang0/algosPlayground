package stack

import (
	"testing"
)

func TestPop(t *testing.T) {
	stk := stack{&[]int{3, 6, 77, 2}}
	lenbefore := len(*stk.s)
	result, err := stk.pop()
	if err != nil {
		t.Error(err)
	}
	expected := 2
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	expected = 77
	stks := *stk.s
	result = stks[len(*stk.s)-1]
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	lenafter := len(*stk.s)
	expectedlen := lenbefore - 1
	if expectedlen != lenafter {
		t.Errorf("expected length %v got %v", expectedlen, lenafter)
	}
	stk = stack{&[]int{}}
	result, err = stk.pop()
	if err == nil {
		t.Errorf("Error should have been thrown for calling pop on zero length stack")
	}
}

func TestPush(t *testing.T) {
	stk := stack{&[]int{3, 6, 77, 2}}
	lenbefore := len(*stk.s)
	stk.push(32)
	expected := 32
	stks := *stk.s
	result := stks[len(*stk.s)-1]
	if result != expected {
		t.Errorf("expected %v got %v", expected, result)
	}
	lenafter := len(*stk.s)
	expectedlen := lenbefore + 1
	if expectedlen != lenafter {
		t.Errorf("expected length %v got %v", expectedlen, lenafter)
	}
}
