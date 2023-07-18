package frequencysort

// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge/537/week-4-may-22nd-may-28th/

type cdata struct {
	freq int
	c    string
}

// V3 []cdata contains always ordered slice of characters and their frequencies
//	  map[string]int used to identify index of character in []cdata

// Runtime: 508 ms-- beats 21.84% of golang submissions-- need better algorithm
// Memory Usage: 8.6 MB
// https://leetcode.com/submissions/detail/344288896/?from=/explore/challenge/card/may-leetcoding-challenge/537/week-4-may-22nd-may-28th/3337/

func frequencySort(s string) string {
	// chararray will always contain sorted (decreasing) cdata structs by frequency
	chararray := []cdata{}
	chararraylocations := map[string]int{} // contains the index of each character in chararray
	for _, r := range s {
		currchar := string(r)
		if _, ok := chararraylocations[currchar]; ok {
			// increment counter of character
			chararray[chararraylocations[currchar]].freq++
			// get index of currchar in chararray
			index := chararraylocations[currchar]
			if index > 0 {
				for index > 0 && chararray[index-1].freq < chararray[index].freq {
					// swap char value & reference variables
					previndexchar := chararray[index-1].c
					chararraylocations[previndexchar], chararraylocations[currchar] = chararraylocations[currchar], chararraylocations[previndexchar]
					chararray[index-1], chararray[index] = chararray[index], chararray[index-1]
					index--
				}
			}
		} else {
			// if character hasn't been encountered
			newchar := cdata{c: currchar, freq: 1}
			chararray = append(chararray, newchar)
			chararraylocations[currchar] = len(chararray) - 1
		}
	}
	// assemble return string
	var rstring string
	for _, cstruct := range chararray {
		for i := 0; i < cstruct.freq; i++ {
			rstring += cstruct.c
		}
	}
	return rstring
}
