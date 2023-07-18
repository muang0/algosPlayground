package validparentheses

// https://leetcode.com/problems/valid-parentheses/

// Runtime: 0 ms, faster than 100.00% of Go online submissions for Valid Parentheses.
// Memory Usage: 2.1 MB, less than 30.00% of Go online submissions for Valid Parentheses.

// Stack data structure, has pop & push methods
type Stack struct {
	s []string
}

func isValid(s string) bool {
	// solve using stack solution
	relation := map[string]string{"{": "}", "(": ")", "[": "]"}
	stk := new(Stack)
	for _, char := range s {
		char := string(char)
		if char == "(" || char == "{" || char == "[" {
			stk.push(string(char))
		} else if len(stk.s) == 0 {
			return false
		} else {
			if char != relation[stk.pop()] {
				return false
			}
		}
	}
	if len(stk.s) != 0 {
		return false
	}
	return true
}

func (stk *Stack) pop() string {
	lenstack := len(stk.s)
	s := stk.s[lenstack-1]
	stk.s = stk.s[:lenstack-1]
	return s
}

func (stk *Stack) push(str string) {
	stk.s = append(stk.s, str)
	return
}
