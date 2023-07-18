package longestcommprefix

// https://leetcode.com/problems/longest-common-prefix/

// Runtime: 0 ms, faster than 100.00% of Go online submissions for Longest Common Prefix.
// Memory Usage: 2.4 MB, less than 75.00% of Go online submissions for Longest Common Prefix.

func lcp(strs []string) string {
	index := 0
	if len(strs) == 0 {
		return ""
	} else if len(strs) == 1 {
		return strs[0]
	}
	for {
		if len(strs[0]) <= index {
			return strs[0][:index]
		}
		for i := 1; i < len(strs); i++ {
			strlen := len(strs[i])
			if index < strlen {
				if strs[i][index] != strs[i-1][index] {
					return strs[0][:index]
				}
			} else {
				return strs[0][:index]
			}
		}
		index++
	}
}
