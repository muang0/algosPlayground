package validparentheses

import (
	"testing"
)

func TestIsValid(t *testing.T) {
	actual := isValid("{()}")
	if actual != true {
		t.Errorf("expected %v got %v", true, actual)
	}
	actual = isValid("{(")
	if actual != false {
		t.Errorf("expected %v got %v", false, actual)
	}
	actual = isValid("")
	if actual != true {
		t.Errorf("expected %v got %v", true, actual)
	}
}

func TestPush(t *testing.T) {
	stk := Stack{s: []string{"test", "sdf"}}
	actual := "s"
	stk.push(actual)
	expected := "s"
	if len(stk.s) != 3 {
		t.Errorf("expected %+v got %+v", []string{"test", "sdf", actual}, stk)
		return
	}
	if stk.s[2] != expected {
		t.Errorf("expected %v got %v", expected, stk.s[2])
	}
}

func TestPop(t *testing.T) {
	expected := "sdf"
	stk := Stack{s: []string{"test", expected}}
	s := stk.pop()
	if s != expected {
		t.Errorf("expected %v got %v", expected, s)
	}
}
