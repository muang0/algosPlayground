package stack

import (
	"errors"
)

type stack struct {
	s *[]int
}

func (stk *stack) pop() (int, error) {
	if len(*stk.s) == 0 {
		return 0, errors.New("Stack is currently empty")
	}
	stks := *stk.s
	val := stks[len(*stk.s)-1]
	*stk.s = stks[:len(*stk.s)-1]
	return val, nil
}

func (stk *stack) push(val int) {
	*stk.s = append(*stk.s, val)
}
